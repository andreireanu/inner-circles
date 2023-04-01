multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getUserToken)]
    #[storage_mapper("userToken")]
    fn user_token(&self, user: &ManagedAddress) -> SingleValueMapper<TokenIdentifier>;

    #[view(getUserSft)]
    #[storage_mapper("userSft")]
    fn user_sft(&self, user: &ManagedAddress) -> SingleValueMapper<TokenIdentifier>;

    #[view(getUserNft)]
    #[storage_mapper("userNft")]
    fn user_nft(&self, user: &ManagedAddress) -> SingleValueMapper<TokenIdentifier>;
}
