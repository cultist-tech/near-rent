#!/bin/bash
source ../scripts/neardev/dev-account.env

TOKEN_ID="_7"

near view $CONTRACT_NAME rent --accountId $CONTRACT_NAME "{ \"contract_id\": \"$NFT_CONTRACT_NAME\", \"token_id\": \"$TOKEN_ID\" }"
