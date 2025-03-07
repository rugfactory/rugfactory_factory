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
# Initialize with custom metadata
near call <contract-id> new '{
    "owner_id": "<owner-account>",
    "total_supply": "1000000000000000000000000000000",
    "metadata": {
        "spec": "ft-1.0.0",
        "name": "My Token",
        "symbol": "TOKEN",
        "icon": "data:image/svg+xml;base64,PHN2ZyBpZD0iU1VORlVOX1JPVU5EX0lDT04iIHZpZXdCb3g9IjAgMCAxMDgwIDEwODAiIHByZXNlcnZlQXNwZWN0UmF0aW89InhNaWRZTWlkIG1lZXQiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CiAgPHJlY3Qgd2lkdGg9IjEwODAiIGhlaWdodD0iMTA4MCIgZmlsbD0iI0IzOTU3MCIvPgogIDxjaXJjbGUgY3g9IjU0MCIgY3k9IjU0MCIgcj0iMzAwIiBmaWxsPSIjMzgyQzFGIiAvPgo8L3N2Zz4=",
        "decimals": 24
    }
}' --accountId <owner-account>
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

