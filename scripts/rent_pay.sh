#!/bin/bash
source ../scripts/neardev/dev-account.env

CALLER_ID="mfight.testnet"
TOKEN_ID="_7"
TIME=3600000000000
PRICE="2"

near call $CONTRACT_NAME rent_pay --accountId $CALLER_ID "{ \"contract_id\": \"$NFT_CONTRACT_NAME\", \"token_id\": \"$TOKEN_ID\", \"time\": $TIME, \"receiver_id\": \"$CALLER_ID\" }" --amount "$PRICE" --gas 300000000000000
