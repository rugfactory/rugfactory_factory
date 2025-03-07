# Deploying, Initializing and Testing Greeting Methods

## Building the Contract

First, build the contract using Cargo:

```bash
# Build the contract
cargo build
cargo near build

# Or use the provided build scripts
./build_cargo.sh
./build_reproducible.sh
```

## Deploying the Contract

Deploy the contract to your NEAR account:

```bash
# Deploy using cargo near
cargo near deploy build-reproducible-wasm <your-account>.testnet

# Or deploy using NEAR CLI
near deploy <your-account>.testnet res/rugfactory.wasm
```

## Initializing the Contract

After deployment, initialize the contract with required addresses:

```bash
# Initialize with testnet addresses
near call <your-account>.testnet init '{"owner_id": "<your-account>.testnet", "ref_contract": "ref-finance-101.testnet", "wrap_near_contract": "wrap.testnet", "shit_token_contract": "shit-237.factory.v10.meme-cooking.testnet"}' --accountId <your-account>.testnet

# Initialize with mainnet addresses
near call <your-account>.near init '{"owner_id": "<your-account>.near", "ref_contract": "v2.ref-finance.near", "wrap_near_contract": "wrap.near", "shit_token_contract": "shit-1170.meme-cooking.near"}' --accountId <your-account>.near
```

## Testing Greeting Methods

The contract includes two greeting methods: one to view and one to set. Note that setting the greeting requires payment of 100 SHIT tokens.

### View Current Greeting

```bash
# View the current greeting (free)
near view <your-account>.testnet greeting_get
```

Expected output:
```json
"Hello from rugfactory!"
```

### Set New Greeting

Before setting a new greeting, ensure you have:
1. Deposited SHIT tokens to the contract
2. The contract is registered with the SHIT token

```bash
# First, register the contract with SHIT token if not already done
near call shit-237.factory.v10.meme-cooking.testnet storage_deposit '{"account_id": "<your-account>.testnet"}' --accountId <your-account>.testnet --amount 0.00125

# Deposit SHIT tokens to contract (amount should be more than 100 SHIT)
near call shit-237.factory.v10.meme-cooking.testnet ft_transfer_call '{"receiver_id": "<your-account>.testnet", "amount": "100", "msg": ""}' --accountId <your-account>.testnet --amount 0.000000000000000000000001

# Set new greeting (costs 100 SHIT)
near call <your-account>.testnet greeting_set '{"message": "New greeting from rugfactory!"}' --accountId <your-account>.testnet
```

Expected output after setting greeting:
```json
{
  "status": "Success",
  "message": "Greeting updated. Charged 100 SHIT tokens."
}
```

> Note: All amounts for SHIT tokens are in their smallest unit (18 decimals). For example, 100 SHIT = 100000000000000000000