use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{env, log, near_bindgen, AccountId, NearToken, PanicOnDefault, Promise};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use std::collections::HashMap;
use schemars::JsonSchema;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub creator_id: AccountId,
}

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
    tokens: HashMap<String, TokenMetadata>,
    default_icon: String,
}






#[near_bindgen]
impl Contract {




    /// 👋
    /// Initialization method
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
            tokens: HashMap::new(),
            default_icon: "data:image/svg+xml;base64,PHN2ZyBpZD0iU1VORlVOX1JPVU5EX0lDT04iIHZpZXdCb3g9IjAgMCAxMDgwIDEwODAiIHByZXNlcnZlQXNwZWN0UmF0aW89InhNaWRZTWlkIG1lZXQiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CiAgPHJlY3Qgd2lkdGg9IjEwODAiIGhlaWdodD0iMTA4MCIgZmlsbD0iI0IzOTU3MCIvPgogIDxjaXJjbGUgY3g9IjU0MCIgY3k9IjU0MCIgcj0iMzAwIiBmaWxsPSIjMzgyQzFGIiAvPgo8L3N2Zz4=".to_string(),
        }
    }


     /// 👋
    /// Admin methods

    pub fn admin_update_info(
        &mut self,
        ref_contract: AccountId,
        shit_token: AccountId,
        wrap_near: AccountId,
    ) {
        // Verify caller is the owner
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only the owner can update contract info"
        );

        // Update contract info
        self.ref_contract = ref_contract;
        self.shit_token = shit_token;
        self.wrap_near = wrap_near;

        log!("Contract info updated successfully");
    }

    pub fn admin_get_info(&self) -> (AccountId, AccountId, AccountId, AccountId) {
        (
            self.owner_id.clone(),
            self.ref_contract.clone(),
            self.shit_token.clone(),
            self.wrap_near.clone(),
        )
    }





    


    

    /// 👋
    /// Greeting methods
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










    


    /// 👋
    /// User methods

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
        msg: String,
    ) -> U128 {
        // Verify the token sender is our SHIT token contract
        let token_contract = env::predecessor_account_id();
        if token_contract != self.shit_token {
            env::log_str(&format!(
                "Rejected token deposit from {}, only accepting SHIT token ({})",
                token_contract,
                self.shit_token
            ));
            return amount; // Return all tokens if wrong token contract
        }

        // Try to deserialize the message if provided
        if !msg.is_empty() {
            env::log_str(&format!("Processing deposit message: {}", msg));
        }

        // Update user's SHIT token balance
        let balance = self.user_shit_balances.get(&sender_id).unwrap_or(&0);
        self.user_shit_balances.insert(sender_id.clone(), balance + amount.0);

        env::log_str(&format!(
            "Successfully deposited {} SHIT tokens for {}",
            amount.0,
            sender_id
        ));

        U128(0) // Accept all tokens
    }






    

    /// 👋
    /// Token methods
    pub fn token_create(&mut self, name: String, symbol: String, icon: Option<String>) -> Promise {
        let account_id = env::predecessor_account_id();
        
        // Verify user has enough balance
        let near_balance = self.user_near_balances.get(&account_id).unwrap_or(&0);
        let shit_balance = self.user_shit_balances.get(&account_id).unwrap_or(&0);
        
        assert!(near_balance >= &1_990_000_000_000_000_000_000_000, "Not enough NEAR balance. Need 1.99 NEAR");
        assert!(shit_balance >= &1000, "Not enough SHIT balance. Need 1000 SHIT");

        // Validate symbol (will be used as subaccount name)
        assert!(symbol.chars().all(|c| c.is_ascii_alphanumeric()), "Symbol must be alphanumeric");
        assert!(symbol.len() <= 20, "Symbol too long");
        
        // Validate icon
        let icon_data = icon.unwrap_or_else(|| self.default_icon.clone());
        assert!(icon_data.len() <= 1024, "Icon data URL too large");

        // Deduct fees
        self.user_near_balances.insert(account_id.clone(), near_balance - 1_990_000_000_000_000_000_000_000);
        self.user_shit_balances.insert(account_id.clone(), shit_balance - 1000);

        // Create metadata
        let metadata = TokenMetadata {
            name: name.clone(),
            symbol: symbol.clone(),
            icon: Some(icon_data.clone()),
            creator_id: account_id.clone(),
        };

        // Store token info
        self.tokens.insert(symbol.clone(), metadata);

        // Create subaccount and deploy token contract
        let subaccount = format!("{}.{}", symbol, env::current_account_id());
        let subaccount_id: AccountId = subaccount.parse().unwrap();

        // Read token contract bytes
        let wasm_bytes = include_bytes!("../res/fungible_token.wasm").to_vec();

        // Deploy and initialize token contract
        Promise::new(subaccount_id.clone())
            .create_account()
            .transfer(NearToken::from_yoctonear(1_900_000_000_000_000_000_000_000)) // 1.9 NEAR
            .deploy_contract(wasm_bytes)
            .function_call(
                "new".to_string(),
                json!({
                    "owner_id": env::current_account_id(),
                    "total_supply": "1000000000000000000000000000", // 1 billion with 24 decimals
                    "metadata": {
                        "spec": "ft-1.0.0",
                        "name": name,
                        "symbol": symbol,
                        "icon": icon_data,
                        "decimals": 24
                    }
                }).to_string().into_bytes(),
                NearToken::from_near(0),
                near_sdk::Gas::from_tgas(30)
            )
            // Register creator account with token contract
            .then(
                Promise::new(subaccount_id.clone())
                    .function_call(
                        "storage_deposit".to_string(),
                        json!({"account_id": account_id}).to_string().into_bytes(),
                        NearToken::from_near(0),
                        near_sdk::Gas::from_tgas(30)
                    )
            )
            // Transfer full supply to creator
            .then(
                Promise::new(subaccount_id.clone())
                    .function_call(
                        "ft_transfer".to_string(),
                        json!({
                            "receiver_id": account_id,
                            "amount": "1000000000000000000000000000"
                        }).to_string().into_bytes(),
                        NearToken::from_yoctonear(1),
                        near_sdk::Gas::from_tgas(30)
                    )
            )
            // Register token with REF Finance
            .then(
                Promise::new(self.ref_contract.clone())
                    .function_call(
                        "storage_deposit".to_string(),
                        json!({"account_id": subaccount_id}).to_string().into_bytes(),
                        NearToken::from_near(1),
                        near_sdk::Gas::from_tgas(30)
                    )
            )
            // Create pool with wrapped NEAR
            .then(
                Promise::new(self.ref_contract.clone())
                    .function_call(
                        "add_simple_pool".to_string(),
                        json!({
                            "tokens": [subaccount_id.to_string(), self.wrap_near.to_string()],
                            "fee": 25
                        }).to_string().into_bytes(),
                        NearToken::from_near(0),
                        near_sdk::Gas::from_tgas(30)
                    )
            )
    }

    pub fn token_list_all(&self) -> Vec<(String, TokenMetadata)> {
        self.tokens.iter().map(|(k, v)| (k.to_string(), v.to_owned())).collect::<Vec<(String, TokenMetadata)>>()
    }

    pub fn token_delete(&mut self, token_symbol: String) -> Promise {
        let account_id = env::predecessor_account_id();
        
        // Get token metadata and verify caller is creator
        let metadata = self.tokens.get(&token_symbol)
            .expect("Token not found");
        assert_eq!(
            metadata.creator_id,
            account_id,
            "Only token creator can delete the token"
        );

        // Create subaccount ID
        let subaccount = format!("{}.{}", token_symbol, env::current_account_id());
        let subaccount_id: AccountId = subaccount.parse().unwrap();

        // Remove token from list
        self.tokens.remove(&token_symbol);

        // Call token delete method and refund NEAR
        Promise::new(subaccount_id)
            .function_call(
                "rugfactory_token_delete".to_string(),
                "".into(),
                NearToken::from_near(0),
                near_sdk::Gas::from_tgas(30)
            )
            .then(
                Promise::new(account_id)
                    .transfer(NearToken::from_yoctonear(1_500_000_000_000_000_000_000_000)) // 1.5 NEAR
            )
    }
}


















/// TESTS

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
        contract.user_withdraw_near(U128(500_000_000_000_000_000_000_000)); // 0.5 NEAR
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
        contract.user_withdraw_near(U128(1_000_000_000_000_000_000_000_000)); // 1 NEAR
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

        // Test SHIT token deposit
        contract.user_deposit_shit(U128(1000));
        assert_eq!(
            contract.user_get_shit_balance("user.testnet".parse().unwrap()).0,
            1000
        );

        // Test FT transfer callback
        context.predecessor_account_id = "shit.testnet".parse().unwrap();
        testing_env!(context);

        let unused_tokens = contract.ft_on_transfer(
            "user.testnet".parse().unwrap(),
            U128(500),
            "deposit".to_string(),
        );
        assert_eq!(unused_tokens.0, 0); // All tokens accepted

        assert_eq!(
            contract.user_get_shit_balance("user.testnet".parse().unwrap()).0,
            1500
        );
    }

    #[test]
    #[should_panic(expected = "Only accept SHIT token")]
    fn test_ft_transfer_wrong_token() {
        let mut context = get_context(false);
        context.predecessor_account_id = "wrong.token.testnet".parse().unwrap();
        testing_env!(context);

        let mut contract = Contract::init(
            "owner.testnet".parse().unwrap(),
            "ref.testnet".parse().unwrap(),
            "shit.testnet".parse().unwrap(),
            "wrap.testnet".parse().unwrap(),
        );

        contract.ft_on_transfer(
            "user.testnet".parse().unwrap(),
            U128(500),
            "deposit".to_string(),
        );
    }
}
