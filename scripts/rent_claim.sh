#!/bin/bash
source ../scripts/neardev/dev-account.env

ACCOUNT_ID="mfight.testnet"
TOKEN_ID="_7"

near call $CONTRACT_NAME rent_claim --accountId $ACCOUNT_ID "{ \"contract_id\": \"$NFT_CONTRACT_NAME\", \"token_id\": \"$TOKEN_ID\" }" --gas 300000000000000
