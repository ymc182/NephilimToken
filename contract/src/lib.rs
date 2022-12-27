use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::store::LazyOption;
use near_sdk::PromiseOrValue;
use near_sdk::{env, AccountId, Balance, BorshStorageKey};
use near_sdk::{log, near_bindgen, require, PanicOnDefault};
mod constants;
use constants::*;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    FungibleToken,
    Metadata,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
    owner_id: AccountId,
}
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ContractV1 {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}
// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let metadata = FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: NAME.to_string(),
            symbol: SYMBOL.to_string(),
            icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
            reference: None,
            reference_hash: None,
            decimals: DECIMALS,
        };
        Self {
            token: FungibleToken::new(StorageKey::FungibleToken),
            metadata: LazyOption::new(StorageKey::Metadata, Some(metadata)),
            owner_id,
        }
    }

    #[init(ignore_state)]
    pub fn migrate(owner_id: AccountId) -> Self {
        let old_state: ContractV1 = env::state_read().expect("failed");
        Self {
            token: old_state.token,
            metadata: old_state.metadata,
            owner_id,
        }
    }
    pub fn update_metadata(&mut self) {
        let metadata = FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: NAME.to_string(),
            symbol: SYMBOL.to_string(),
            icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
            reference: None,
            reference_hash: None,
            decimals: DECIMALS,
        };
        self.metadata = LazyOption::new(StorageKey::Metadata, Some(metadata));
    }
    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }
    pub fn ft_mint(&mut self, amount: U128) {
        require!(
            self.owner_id == env::predecessor_account_id(),
            "Only owner can mint"
        );
        self.token
            .internal_deposit(&env::signer_account_id(), amount.0);
    }
}
near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().as_ref().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let _contract = Contract::new(env::signer_account_id());
        println!("{:?}", env::signer_account_id());
    }
}
