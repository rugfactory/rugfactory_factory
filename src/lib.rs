use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{env, log, near_bindgen, AccountId, NearToken, PanicOnDefault, Promise};
use std::collections::HashMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    ref_contract: AccountId,
    shit_token: AccountId,
    wrap_near: AccountId,
    greeting: String,
    user_near_balances: HashMap<AccountId, u128>,
    user_shit_balances: HashMap<AccountId, u128>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(
        owner_id: AccountId,
        ref_contract: AccountId,
        shit_token: AccountId,
        wrap_near: AccountId,
    ) -> Self {
        Self {
            owner_id,
            ref_contract,
            shit_token,
            wrap_near,
            greeting: "Hello".to_string(),
            user_near_balances: HashMap::new(),
            user_shit_balances: HashMap::new(),
        }
    }

    // Greeting methods
    pub fn greeting_get(&self) -> String {
        self.greeting.clone()
    }

    pub fn greeting_set(&mut self, greeting: String) {
        // Verify user has enough SHIT token balance
        let account_id = env::predecessor_account_id();
        let balance = self.user_shit_balances.get(&account_id).unwrap_or(&0);
        assert!(
            balance >= &100,
            "Not enough SHIT token balance. Need 100 SHIT to update greeting"
        );

        // Deduct SHIT tokens
        self.user_shit_balances.insert(account_id, balance - 100);

        log!("Saving greeting: {}", greeting);
        self.greeting = greeting;
    }

    // User deposit methods
    #[payable]
    pub fn user_deposit_near(&mut self) {
        let account_id = env::predecessor_account_id();
        let deposit = env::attached_deposit().as_yoctonear() as u128;
        let balance = self.user_near_balances.get(&account_id).unwrap_or(&0);
        self.user_near_balances.insert(account_id, balance + deposit);
    }

    pub fn user_deposit_shit(&mut self, amount: U128) {
        let account_id = env::predecessor_account_id();
        let balance = self.user_shit_balances.get(&account_id).unwrap_or(&0);
        self.user_shit_balances.insert(account_id, balance + amount.0);
    }

    // User balance view methods
    pub fn user_get_near_balance(&self, account_id: AccountId) -> U128 {
        U128(*self.user_near_balances.get(&account_id).unwrap_or(&0))
    }

    pub fn user_get_shit_balance(&self, account_id: AccountId) -> U128 {
        U128(*self.user_shit_balances.get(&account_id).unwrap_or(&0))
    }

    pub fn user_get_balance(&self, account_id: AccountId) -> (U128, U128) {
        (
            self.user_get_near_balance(account_id.clone()),
            self.user_get_shit_balance(account_id),
        )
    }

    // User withdrawal method
    pub fn user_withdraw_near(&mut self, amount: U128) -> Promise {
        let account_id = env::predecessor_account_id();
        let balance = self.user_near_balances.get(&account_id).unwrap_or(&0);
        assert!(balance >= &amount.0, "Not enough NEAR balance");

        self.user_near_balances.insert(account_id.clone(), balance - amount.0);
        Promise::new(account_id).transfer(NearToken::from_yoctonear(amount.0))
    }

    // FT transfer callback
    pub fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        _msg: String,
    ) -> U128 {
        assert_eq!(
            env::predecessor_account_id(),
            self.shit_token,
            "Only accept SHIT token"
        );

        let balance = self.user_shit_balances.get(&sender_id).unwrap_or(&0);
        self.user_shit_balances.insert(sender_id, balance + amount.0);

        U128(0) // Accept all tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .current_account_id("contract.testnet".parse().unwrap())
            .signer_account_id("user.testnet".parse().unwrap())
            .predecessor_account_id("user.testnet".parse().unwrap())
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
            200, // More than required 100 SHIT
        );

        assert_eq!(contract.greeting_get(), "Hello");
        contract.greeting_set("Howdy".to_string());
        assert_eq!(contract.greeting_get(), "Howdy");

        // Check SHIT balance was deducted
        assert_eq!(
            contract.user_get_shit_balance("user.testnet".parse().unwrap()).0,
            100
        );
    }
}
