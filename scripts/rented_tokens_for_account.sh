#!/bin/bash
source ../scripts/neardev/dev-account.env

ACCOUNT_ID="mfight.testnet"
FROM_INDEX=0
LIMIT=1000

near view $CONTRACT_NAME rented_tokens_for_account "{ \"account_id\": \"$ACCOUNT_ID\", \"from_index\": \"$FROM_INDEX\", \"limit\": $LIMIT }"
