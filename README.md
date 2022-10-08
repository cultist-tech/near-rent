# NFT RENT

Реализовать возможность передавать NFT на определенное время. По истечению срока аренды, NFT возвращается её холдеру. Сделать возможность оплачивать аренду за любой FT или NEAR.

```rust
pub trait RentFactoryCore {
  // список
  fn rent_token_is_locked(&self, nft_contract_id: AccountId, token_id: TokenId) -> bool;

  // обновить оффер
  fn rent_update(&mut self, nft_contract_id: AccountId, token_id: TokenId, price_per_hour: U128, min_time: u64, max_time: u64);
  // убрать оффер с маркета
  fn rent_remove(&mut self, nft_contract_id: AccountId, token_id: TokenId);

  // оплатить аренду (купить)
  fn rent_pay(&mut self, nft_contract_id: AccountId, token_id: TokenId, time: u64, receiver_id: AccountId) -> Promise;
  // забрать токен после аренды
  fn rent_claim(&mut self, nft_contract_id: AccountId, token_id: TokenId) -> Promise;

  // узнать закончилась ли аренда
  fn rent_is_ended(&self, nft_contract_id: AccountId, token_id: TokenId) -> bool;
  // кол-во офферов аренды
  fn rent_total_supply(&self) -> u64;

  fn rent_is_approved(&self, nft_contract_id: AccountId, token_id: TokenId, account_id: AccountId) -> bool;
}
```
