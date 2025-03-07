# Token Methods Guide

This guide explains how to interact with the token creation and management methods in the rugfactory contract.

## Token Creation

### Prerequisites
- You must have sufficient balances:
  - 1.99 NEAR
  - 1000 SHIT tokens

### Creating a New Token

```bash
# Create a new token with required parameters
near call <contract-id> token_create '{
    "name": "My Token",
    "symbol": "TOKEN",
    "icon": null  # Optional: provide base64 encoded SVG/PNG (max 1KB)
}' --accountId <your-account>.testnet
```

When you create a token:
1. The contract deducts:
   - 1.99 NEAR from your NEAR balance
   - 1000 SHIT from your SHIT balance
2. A new subaccount is created (e.g., token.rugfactory.testnet)
3. The token contract is deployed with:
   - 24 decimal places
   - 1 billion total supply
   - Your specified metadata (name, symbol, icon)
4. The contract automatically:
   - Registers your account with the new token
   - Transfers the full token supply to you
   - Registers the token with REF Finance
   - Creates a liquidity pool with wrapped NEAR

### Important Notes
- Symbol must be alphanumeric and maximum 20 characters
- If no icon is provided, a default icon will be used
- Custom icons must be less than 1KB in size
- The contract becomes the owner of the token contract

## Token Deletion

### Prerequisites
- You must be the original creator of the token

```bash
# Delete a token you created
near call <contract-id> token_delete '{"token_symbol": "TOKEN"}' --accountId <your-account>.testnet
```

When you delete a token:
1. The token is removed from the contract's registry
2. The token contract is deleted
3. You receive a refund of 1.5 NEAR

### Important Notes
- Only the original creator can delete their token
- SHIT tokens used for creation are not refunded
- The difference in NEAR (0.49 NEAR) is kept as a fee

---

For more information about the token contract implementation and REF Finance integration, see the [new-token-guide.md](../res/new-token-guide.md) in the resources folder.