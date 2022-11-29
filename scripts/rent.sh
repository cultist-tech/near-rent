#!/bin/bash
source ../scripts/neardev/dev-account.env

TOKEN_ID="14000"

near view $CONTRACT_NAME rent --accountId $CONTRACT_NAME "{ \"contract_id\": \"$NFT_CONTRACT_NAME\", \"token_id\": \"$TOKEN_ID\" }"
