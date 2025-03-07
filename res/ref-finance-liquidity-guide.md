# Ref Finance Liquidity Operations Guide

This guide covers how to interact with Ref Finance's liquidity pools using NEAR CLI, including operations on both testnet and mainnet.

## Prerequisites

- NEAR CLI installed (`npm install -g near-cli`)
- NEAR account with sufficient balance (at least 1 NEAR for storage deposits)
- Basic understanding of DeFi concepts

## Environment Setup

### Testnet
```bash
export NEAR_ENV=testnet
near login
```

### Mainnet
```bash
export NEAR_ENV=mainnet
near login
```

## Contract Addresses
- Testnet: `ref-finance-101.testnet`
- Mainnet: `v2.ref-finance.near`

## Initial Setup

### Register with Ref Finance (Required once)

#### Testnet
```bash
near call ref-finance-101.testnet storage_deposit '' --accountId YOUR_ACCOUNT.testnet --deposit 1
```

#### Mainnet
```bash
near call v2.ref-finance.near storage_deposit '' --accountId YOUR_ACCOUNT.near --deposit 1
```

## Creating a New Pool

### Simple Pool (Testnet)
```bash
near call ref-finance-101.testnet add_simple_pool '{"tokens": ["token1.testnet", "token2.testnet"], "fee": 25}' --accountId YOUR_ACCOUNT.testnet --deposit 0.1
```

### Simple Pool (Mainnet)
```bash
near call v2.ref-finance.near add_simple_pool '{"tokens": ["token1.near", "token2.near"], "fee": 25}' --accountId YOUR_ACCOUNT.near --deposit 0.1
```

Note: Fee is in basis points (25 = 0.25%)

## Adding Liquidity

### Register Tokens (Required once per token)

#### Testnet
```bash
near call token1.testnet storage_deposit '{"account_id": "YOUR_ACCOUNT.testnet", "registration_only": true}' --accountId YOUR_ACCOUNT.testnet --deposit 0.0125
near call token2.testnet storage_deposit '{"account_id": "YOUR_ACCOUNT.testnet", "registration_only": true}' --accountId YOUR_ACCOUNT.testnet --deposit 0.0125
```

#### Mainnet
```bash
near call token1.near storage_deposit '{"account_id": "YOUR_ACCOUNT.near", "registration_only": true}' --accountId YOUR_ACCOUNT.near --deposit 0.0125
near call token2.near storage_deposit '{"account_id": "YOUR_ACCOUNT.near", "registration_only": true}' --accountId YOUR_ACCOUNT.near --deposit 0.0125
```

### Register Tokens with Ref Finance

#### Testnet
```bash
near call ref-finance-101.testnet register_tokens '{"token_ids": ["token1.testnet", "token2.testnet"]}' --accountId YOUR_ACCOUNT.testnet --deposit 0.1
```

#### Mainnet
```bash
near call v2.ref-finance.near register_tokens '{"token_ids": ["token1.near", "token2.near"]}' --accountId YOUR_ACCOUNT.near --deposit 0.1
```

### Add Liquidity

#### Testnet
```bash
near call token1.testnet ft_transfer_call '{"receiver_id": "ref-finance-101.testnet", "amount": "1000000000000000000", "msg": ""}' --accountId YOUR_ACCOUNT.testnet --depositYocto 1 --gas 300000000000000
near call token2.testnet ft_transfer_call '{"receiver_id": "ref-finance-101.testnet", "amount": "1000000000000000000", "msg": ""}' --accountId YOUR_ACCOUNT.testnet --depositYocto 1 --gas 300000000000000

near call ref-finance-101.testnet add_liquidity '{"pool_id": POOL_ID, "amounts": ["1000000000000000000", "1000000000000000000"]}' --accountId YOUR_ACCOUNT.testnet --depositYocto 1 --gas 300000000000000
```

#### Mainnet
```bash
near call token1.near ft_transfer_call '{"receiver_id": "v2.ref-finance.near", "amount": "1000000000000000000", "msg": ""}' --accountId YOUR_ACCOUNT.near --depositYocto 1 --gas 300000000000000
near call token2.near ft_transfer_call '{"receiver_id": "v2.ref-finance.near", "amount": "1000000000000000000", "msg": ""}' --accountId YOUR_ACCOUNT.near --depositYocto 1 --gas 300000000000000

near call v2.ref-finance.near add_liquidity '{"pool_id": POOL_ID, "amounts": ["1000000000000000000", "1000000000000000000"]}' --accountId YOUR_ACCOUNT.near --depositYocto 1 --gas 300000000000000
```

## Viewing Liquidity Positions

### Get Pool Information

#### Testnet
```bash
near view ref-finance-101.testnet get_pool '{"pool_id": POOL_ID}'
```

#### Mainnet
```bash
near view v2.ref-finance.near get_pool '{"pool_id": POOL_ID}'
```

### View Your Shares

#### Testnet
```bash
near view ref-finance-101.testnet get_pool_shares '{"pool_id": POOL_ID, "account_id": "YOUR_ACCOUNT.testnet"}'
```

#### Mainnet
```bash
near view v2.ref-finance.near get_pool_shares '{"pool_id": POOL_ID, "account_id": "YOUR_ACCOUNT.near"}'
```

## Removing Liquidity

### Remove All Liquidity

#### Testnet
```bash
near call ref-finance-101.testnet remove_liquidity '{"pool_id": POOL_ID, "shares": "TOTAL_SHARES", "min_amounts": ["0", "0"]}' --accountId YOUR_ACCOUNT.testnet --depositYocto 1 --gas 300000000000000
```

#### Mainnet
```bash
near call v2.ref-finance.near remove_liquidity '{"pool_id": POOL_ID, "shares": "TOTAL_SHARES", "min_amounts": ["0", "0"]}' --accountId YOUR_ACCOUNT.near --depositYocto 1 --gas 300000000000000
```

### Remove Partial Liquidity

#### Testnet
```bash
near call ref-finance-101.testnet remove_liquidity '{"pool_id": POOL_ID, "shares": "PARTIAL_SHARES", "min_amounts": ["MIN_AMOUNT1", "MIN_AMOUNT2"]}' --accountId YOUR_ACCOUNT.testnet --depositYocto 1 --gas 300000000000000
```

#### Mainnet
```bash
near call v2.ref-finance.near remove_liquidity '{"pool_id": POOL_ID, "shares": "PARTIAL_SHARES", "min_amounts": ["MIN_AMOUNT1", "MIN_AMOUNT2"]}' --accountId YOUR_ACCOUNT.near --depositYocto 1 --gas 300000000000000
```

## Important Notes

1. Always verify contract addresses before interacting
2. Test with small amounts first
3. Keep track of your pool IDs
4. Ensure sufficient NEAR balance for gas fees (at least 1 NEAR recommended)
5. Remember to register tokens with both the token contract and Ref Finance
6. Use appropriate slippage protection with `min_amounts`
7. Mainnet operations are irreversible
8. Gas limits may need adjustment based on network conditions

## Common Issues and Solutions

1. **Insufficient Storage**: Ensure you've deposited enough NEAR for storage (1 NEAR recommended)
2. **Token Registration**: Register tokens with both token contracts and Ref Finance
3. **Gas Errors**: Increase gas limit using --gas flag
4. **Failed Transactions**: Double-check pool IDs and token addresses
5. **Transfer Errors**: Ensure proper use of ft_transfer_call instead of ft_approve