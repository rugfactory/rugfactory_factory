use serde_json::json;

#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
    let contract_wasm = near_workspaces::compile_project("./").await?;

    test_basics_on(&contract_wasm).await?;
    Ok(())
}

async fn test_basics_on(contract_wasm: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract = sandbox.dev_deploy(contract_wasm).await?;

    let user_account = sandbox.dev_create_account().await?;

    // Initialize the contract first
    let outcome = contract
        .call("init")
        .args_json(json!({
            "owner_id": contract.id(),
            "ref_contract": contract.id(),
            "shit_token": contract.id(),
            "wrap_near": contract.id()
        }))
        .transact()
        .await?;
    assert!(outcome.is_success());

    // Add some SHIT balance for the test account
    let outcome = contract
        .call("user_deposit_shit")
        .args_json(json!({"amount": "200"}))
        .transact()
        .await?;
    assert!(outcome.is_success());

    let outcome = user_account
        .call(contract.id(), "greeting_set")
        .args_json(json!({"greeting": "Hello World!"}))
        .transact()
        .await?;
    assert!(outcome.is_success());

    let user_message_outcome = contract.view("greeting_get").args_json(json!({})).await?;
    assert_eq!(user_message_outcome.json::<String>()?, "Hello World!");

    Ok(())
}


