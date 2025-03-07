# Fungible Token Guide for Rugfactory

This guide explains how the rugfactory contract creates and manages fungible tokens through subaccounts.

## Token Creation Process

When a new token is created through the rugfactory contract:

1. A new subaccount is created (e.g., token.rugfun.testnet)
2. The fungible token contract is deployed to this subaccount
3. The contract is initialized with standard parameters:
   - 24 decimal places
   - 1 billion total supply (1,000,000,000,000,000,000,000,000,000,000 yocto tokens)
   - Default metadata with project-specific icon

## Initialization Parameters

```bash
# Standard initialization with default metadata
new_default_meta {
    "owner_id": "<rugfactory-contract>",
    "total_supply": "1000000000000000000000000000000"
}
```

## Rugfactory-Specific Methods

The token contract includes two special methods for rugfactory management:

```bash
# Check the current owner of the token contract
near view <token-contract> rugfactory_owner_check

# Delete the token contract (owner-only method)
near call <token-contract> rugfactory_token_delete '{}' --accountId <owner-account>
```

Note: The rugfactory contract automatically handles all token creation, initialization, and management. Users don't need to call these methods directly.

---

copyright: 2025 by sleet.near, in partnership with huggies.near

