#!/bin/bash
source ../scripts/neardev/dev-account.env

near view $CONTRACT_NAME rents_supply_by_contract "{ \"contract_id\": \"$NFT_CONTRACT_NAME\" }"
