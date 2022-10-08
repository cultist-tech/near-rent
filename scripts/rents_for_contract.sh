#!/bin/bash
source ../scripts/neardev/dev-account.env

ACCOUNT_ID="$NFT_CONTRACT_NAME"
FROM_INDEX=0
LIMIT=1000

near view $CONTRACT_NAME rents_by_contract "{ \"contract_id\": \"$ACCOUNT_ID\", \"from_index\": \"$FROM_INDEX\", \"limit\": $LIMIT }"
