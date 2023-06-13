multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getCreatorToken)]
    #[storage_mapper("creatorToken")]
    fn creator_token(&self, user: &ManagedAddress) -> SingleValueMapper<TokenIdentifier>;

    #[view(getCreatorSft)]
    #[storage_mapper("creatorSft")]
    fn creator_sft(&self, user: &ManagedAddress) -> SingleValueMapper<TokenIdentifier>;
}
