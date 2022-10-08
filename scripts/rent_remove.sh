#!/bin/bash
source ../scripts/neardev/dev-account.env

TOKEN_ID="_7"
near call $CONTRACT_NAME rent_remove --accountId $ACCOUNT_ID "{ \"token_id\": \"$TOKEN_ID\", \"contract_id\": \"$NFT_CONTRACT_NAME\", \"account_id\": \"$ACCOUNT_ID\" }"
