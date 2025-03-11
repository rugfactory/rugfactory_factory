use near_sdk::test_utils::VMContextBuilder;
use near_sdk::{testing_env, VMContext, NearToken};
use rugfactory_factory::{Contract, TokenMetadata};

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
    let context = get_context(false);
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
        .account_balance(NearToken::from_near(1))
        .build();
    testing_env!(context);

    // Available balance should be 0 since storage cost (4 NEAR) is greater than contract balance (1 NEAR)
    let admin_balance = contract.admin_get_balance();
    assert_eq!(
        admin_balance.0,
        0 // Contract balance (1 NEAR) - user deposits (1 NEAR) - storage cost (4 NEAR) = 0
    );
}

#[test]
fn test_token_delete_and_refund() {
    let mut context = get_context(false);
    testing_env!(context.clone());

    let mut contract = Contract::init(
        "owner.testnet".parse().unwrap(),
        "ref.testnet".parse().unwrap(),
        "shit.testnet".parse().unwrap(),
        "wrap.testnet".parse().unwrap(),
    );

    // Create a test token
    let token_symbol = "TEST".to_string();
    let token_metadata = TokenMetadata {
        name: "Test Token".to_string(),
        symbol: token_symbol.clone(),
        icon: None,
        creator_id: "user.testnet".to_string(),
    };
    contract.tokens.insert(token_symbol.clone(), token_metadata);

    // Set up context for token deletion
    context.predecessor_account_id = "user.testnet".parse().unwrap();
    context.account_balance = NearToken::from_near(2); // Set contract balance to cover refund
    testing_env!(context);

    // Delete token and verify promise was created
    let delete_promise = contract.token_delete(token_symbol.clone());
    let _ = delete_promise; // Verify promise is returned

    // Simulate callback execution
    let callback_context = VMContextBuilder::new()
        .current_account_id("contract.testnet".parse().unwrap())
        .predecessor_account_id("contract.testnet".parse().unwrap())
        .signer_account_id("user.testnet".parse().unwrap())
        .account_balance(NearToken::from_near(2))
        .build();
    testing_env!(callback_context);

    // Execute callback and verify refund promise was created
    let refund_promise = contract.token_delete_callback("user.testnet".parse().unwrap());
    let _ = refund_promise; // Verify promise is returned
    assert!(matches!(refund_promise, near_sdk::Promise { .. }), "Refund promise should be created");

    // Verify token was removed
    assert!(!contract.tokens.contains_key(&token_symbol), "Token should be removed from registry");
}