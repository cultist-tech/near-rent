#!/bin/bash
source ../scripts/neardev/dev-account.env

TOKEN_ID="8004"
ACCOUNT_ID="muzikant.testnet"
PRICE=1200000000000000000000000
MIN_TIME=3600000000000
MAX_TIME=3600000000000

near call $CONTRACT_NAME rent_update --accountId $ACCOUNT_ID "{ \"token_id\": \"$TOKEN_ID\", \"contract_id\": \"$NFT_CONTRACT_NAME\", \"ft_token_id\": \"near\", \"price_per_hour\": \"$PRICE\", \"min_time\": $MIN_TIME, \"max_time\": $MAX_TIME }"
