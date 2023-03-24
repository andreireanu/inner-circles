multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getUserToken)]
    #[storage_mapper("userToken")]
    fn user_token(&self, user: &ManagedAddress) -> SingleValueMapper<TokenIdentifier>;
}
