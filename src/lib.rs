use near_sdk::{AccountId, BorshStorageKey, near_bindgen, PanicOnDefault, env};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, TreeMap, UnorderedSet};
use mfight_sdk::rent::{RentFeature, Rent, TokenId};

mod nft_callbacks;
mod ft_callbacks;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  rent: RentFeature,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    RentsCurrent,
    RentsPending,
    RentTokensPerAccount,
    RentsById,
    RentsPerAccount,
    RentsAt,
    RentContractTokens,
    ApprovedOwners
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new_with_default_meta(owner_id: AccountId) -> Self {
    Self::new(
      owner_id
    )
  }

  #[init]
  pub fn new(owner_id: AccountId) -> Self {
      let this = Self {
        rent: RentFeature::new(
          Some(StorageKey::ApprovedOwners),
          StorageKey::RentsCurrent,
          StorageKey::RentsPending,
          StorageKey::RentsById,
          StorageKey::RentTokensPerAccount,
          StorageKey::RentsPerAccount,
          StorageKey::RentsAt,
          StorageKey::RentContractTokens,
        ),
      };

      this
  }

  #[init(ignore_state)]
  #[private]
  pub fn migrate() -> Self {
    #[derive(BorshDeserialize)]
    pub struct OldRentFeature {
      // approved nft tokens
      pub approved_owner_by_id: Option<LookupMap<TokenId, AccountId>>,
      // paid rents
      pub rents_current: TreeMap<TokenId, AccountId>,
      // available rents for pay
      pub rents_pending: UnorderedSet<TokenId>,
      // rents info
      pub rents_by_id: TreeMap<TokenId, Rent>,
      // rents per account
      pub rents_per_account: TreeMap<AccountId, UnorderedSet<TokenId>>,
      // rents start time
      pub rents_end_by_id: LookupMap<TokenId, u64>,
      // rented nft tokens per account
      pub rent_tokens_per_account: LookupMap<AccountId, UnorderedSet<TokenId>>,
      //
      pub rent_tokens_by_contract: TreeMap<AccountId, UnorderedSet<TokenId>>,
    }

    #[derive(BorshDeserialize)]
    struct Old {
      rent: OldRentFeature,
    }

    let old: Old = env::state_read().expect("Error");

    let rent = RentFeature {
      approved_owner_by_id: old.rent.approved_owner_by_id,
      rents_current: old.rent.rents_current,
      rents_pending: old.rent.rents_pending,
      rents_by_id: old.rent.rents_by_id,
      rents_per_account: old.rent.rents_per_account,
      rents_end_by_id: old.rent.rents_end_by_id,
      rent_tokens_per_account: old.rent.rent_tokens_per_account,
      rent_tokens_by_contract: old.rent.rent_tokens_by_contract,
    };

    Self {
      rent,
    }
  }
}

mfight_sdk::impl_rent_core!(Contract, rent);
mfight_sdk::impl_rent_enumeration!(Contract, rent);

