use near_sdk::test_utils::VMContextBuilder;
use near_sdk::{testing_env, VMContext, NearToken};
use rugfactory_factory::Contract;

fn get_context(is_view: bool) -> VMContext {
    VMContextBuilder::new()
        .current_account_id("contract.testnet".parse().unwrap())
        .signer_account_id("user.testnet".parse().unwrap())
        .predecessor_account_id("user.testnet".parse().unwrap())
        .attached_deposit(NearToken::from_near(1))
        .is_view(is_view)
        .build()
}

#[test]
fn test_init() {
    let context = get_context(false);
    testing_env!(context);

    let contract = Contract::init(
        "owner.testnet".parse().unwrap(),
        "ref.testnet".parse().unwrap(),
        "shit.testnet".parse().unwrap(),
        "wrap.testnet".parse().unwrap(),
    );

    assert_eq!(contract.greeting, "Hello");
    assert_eq!(contract.owner_id.to_string(), "owner.testnet");
}

#[test]
fn test_greeting() {
    let context = get_context(false);
    testing_env!(context);

    let mut contract = Contract::init(
        "owner.testnet".parse().unwrap(),
        "ref.testnet".parse().unwrap(),
        "shit.testnet".parse().unwrap(),
        "wrap.testnet".parse().unwrap(),
    );

    // Add some SHIT balance for testing
    contract.user_shit_balances.insert(
        "user.testnet".parse().unwrap(),
        100000000000000000000, // 100 SHIT with 18 decimals
    );

    assert_eq!(contract.greeting_get(), "Hello");
    contract.greeting_set("Howdy".to_string());
    assert_eq!(contract.greeting_get(), "Howdy");

    // Check SHIT balance was deducted
    assert_eq!(
        contract.user_get_shit_balance("user.testnet".parse().unwrap()).0,
        0 // Balance should be 0 after deducting 100 SHIT
    );
}

#[test]
fn test_near_deposit_and_withdrawal() {
    let context = get_context(false);
    testing_env!(context);

    let mut contract = Contract::init(
        "owner.testnet".parse().unwrap(),
        "ref.testnet".parse().unwrap(),
        "shit.testnet".parse().unwrap(),
        "wrap.testnet".parse().unwrap(),
    );

    // Test NEAR deposit
    contract.user_deposit_near();
    assert_eq!(
        contract.user_get_near_balance("user.testnet".parse().unwrap()).0,
        1_000_000_000_000_000_000_000_000 // 1 NEAR
    );

    // Test NEAR withdrawal
    contract.user_withdraw_near(near_sdk::json_types::U128(500_000_000_000_000_000_000_000)); // 0.5 NEAR
    assert_eq!(
        contract.user_get_near_balance("user.testnet".parse().unwrap()).0,
        500_000_000_000_000_000_000_000 // 0.5 NEAR
    );
}

#[test]
#[should_panic(expected = "Not enough NEAR balance")]
fn test_near_withdrawal_insufficient_balance() {
    let context = get_context(false);
    testing_env!(context);

    let mut contract = Contract::init(
        "owner.testnet".parse().unwrap(),
        "ref.testnet".parse().unwrap(),
        "shit.testnet".parse().unwrap(),
        "wrap.testnet".parse().unwrap(),
    );

    // Try to withdraw without depositing
    contract.user_withdraw_near(near_sdk::json_types::U128(1_000_000_000_000_000_000_000_000)); // 1 NEAR
}

#[test]
fn test_shit_token_operations() {
    let mut context = get_context(false);
    testing_env!(context.clone());

    let mut contract = Contract::init(
        "owner.testnet".parse().unwrap(),
        "ref.testnet".parse().unwrap(),
        "shit.testnet".parse().unwrap(),
        "wrap.testnet".parse().unwrap(),
    );

    // Test initial SHIT token balance view
    assert_eq!(
        contract.user_get_shit_balance("user.testnet".parse().unwrap()).0,
        0
    );

    // Test FT transfer callback
    context.predecessor_account_id = "shit.testnet".parse().unwrap();
    testing_env!(context);

    let unused_tokens = contract.ft_on_transfer(
        "user.testnet".parse().unwrap(),
        near_sdk::json_types::U128(500),
        "deposit".to_string(),
    );
    assert_eq!(unused_tokens.0, 0); // All tokens accepted

    // Verify balance after deposit
    assert_eq!(
        contract.user_get_shit_balance("user.testnet".parse().unwrap()).0,
        500
    );
}

#[test]
fn test_admin_get_balance() {
    let mut context = get_context(false);
    testing_env!(context.clone());

    let mut contract = Contract::init(
        "owner.testnet".parse().unwrap(),
        "ref.testnet".parse().unwrap(),
        "shit.testnet".parse().unwrap(),
        "wrap.testnet".parse().unwrap(),
    );

    // Set up test environment with some user deposits
    contract.user_deposit_near(); // Deposits 1 NEAR

    // Set contract balance to 2 NEAR for testing
    let context = VMContextBuilder::new()
        .current_account_id("contract.testnet".parse().unwrap())
        .signer_account_id("user.testnet".parse().unwrap())
        .predecessor_account_id("owner.testnet".parse().unwrap())
        .attached_deposit(NearToken::from_near(1))
        .account_balance(NearToken::from_near(2))
        .build();
    testing_env!(context);

    // Available balance should be: 2 NEAR - 1 NEAR (user deposit) - 0.001 NEAR (storage)
    let admin_balance = contract.admin_get_balance();
    assert_eq!(
        admin_balance.0,
        999_000_000_000_000_000_000_000 // ~0.999 NEAR
    );
}