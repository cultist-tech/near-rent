use near_sdk::{ env, AccountId, near_bindgen, PromiseOrValue };
use crate::*;
use mfight_sdk::nft::NonFungibleTokenApprovalReceiver;
use mfight_sdk::rent::RentOnNftApproveArgs;
use near_sdk::serde::{ Deserialize, Serialize };
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum Args {
    Rent(RentOnNftApproveArgs),
}

#[near_bindgen]
impl NonFungibleTokenApprovalReceiver for Contract {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String
    ) -> PromiseOrValue<String> {
        let nft_contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();

        assert_ne!(
            nft_contract_id,
            signer_id,
            "nft_on_approve should only be called via cross-contract call"
        );
        assert_eq!(&owner_id, &signer_id, "owner_id should be signer_id");

        match near_sdk::serde_json::from_str(&msg).expect("Invalid Args") {
            Args::Rent(rentArgs) => {
                self.rent.internal_on_nft_approve(
                    &rentArgs,
                    &nft_contract_id,
                    &token_id,
                    &owner_id
                )
            }
        }
    }
}
