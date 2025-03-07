# User Methods Guide

This guide covers how to interact with the user-related methods of the rugfactory contract, including depositing tokens, checking balances, and withdrawing NEAR.

## SHIT Token Operations

### Deposit SHIT Tokens

Before depositing SHIT tokens, ensure the contract is registered with the SHIT token:

```bash
# Register contract with SHIT token (if not already done)
near call shit-237.factory.v10.meme-cooking.testnet storage_deposit '{"account_id": "<your-account>.testnet"}' --accountId <your-account>.testnet --deposit 0.00125

# Deposit SHIT tokens to contract
near call shit-237.factory.v10.meme-cooking.testnet ft_transfer_call '{"receiver_id": "<your-account>.testnet", "amount": "1000000000000000000000", "msg": ""}' --accountId <your-account>.testnet --deposit 0.000000000000000000000001
```

### Check SHIT Balance

```bash
# View your SHIT token balance in the contract
near view <your-account>.testnet user_get_shit_balance '{"account_id": "<your-account>.testnet"}'
```

Expected output:
```json
"1000000000000000000000"
```

## NEAR Token Operations

### Deposit NEAR

```bash
# Deposit NEAR to the contract (amount in yoctoNEAR)
near call <your-account>.testnet user_deposit_near '{}' --accountId <your-account>.testnet --deposit 2
```

Expected output:
```json
{
  "status": "Success",
  "message": "Deposited 2 NEAR"
}
```

### Check NEAR Balance

```bash
# View your NEAR balance in the contract
near view <your-account>.testnet user_get_near_balance '{"account_id": "<your-account>.testnet"}'
```

Expected output:
```json
"2000000000000000000000000"
```

### Withdraw NEAR

```bash
# Withdraw NEAR from the contract (amount in yoctoNEAR)
near call <your-account>.testnet user_withdraw_near '{"amount": "1000000000000000000000000"}' --accountId <your-account>.testnet
```

Expected output:
```json
{
  "status": "Success",
  "message": "Withdrawn 1 NEAR"
}
```

## View All Balances

You can view both your NEAR and SHIT token balances in one call:

```bash
# Get both NEAR and SHIT balances
near view <your-account>.testnet user_get_balance '{"account_id": "<your-account>.testnet"}'
```

Expected output:
```json
{
  "near_balance": "1000000000000000000000000",
  "shit_balance": "1000000000000000000000"
}
```

## Important Notes

1. All NEAR amounts are in yoctoNEAR (1 NEAR = 10^24 yoctoNEAR)
2. All SHIT token amounts are in their smallest unit (18 decimals)
3. You cannot withdraw SHIT tokens directly - they are used for contract operations
4. The contract automatically deducts from your balances when performing actions that require payment
5. Ensure you have sufficient balances before performing actions that require payment