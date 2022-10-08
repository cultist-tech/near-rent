#!/bin/bash
source ../scripts/neardev/dev-account.env

TOKEN_ID="_7"
ACCOUNT_ID="muzikant.testnet"
PRICE=1000000000000000000000000
MIN_TIME=3600000000000
MAX_TIME=9000000000000

near call $NFT_CONTRACT_NAME nft_approve --accountId $ACCOUNT_ID "{ \"token_id\": \"$TOKEN_ID\", \"account_id\": \"$CONTRACT_NAME\",\"msg\": \"{ \\\"Rent\\\": { \\\"min_time\\\":$MIN_TIME,\\\"max_time\\\":$MAX_TIME,\\\"sale_conditions\\\":{\\\"near\\\": \\\"$PRICE\\\"}} }\" }"
