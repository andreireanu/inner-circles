multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(PartialEq, TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct Campaign<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub hashtag: ManagedBuffer<M>,
    pub amount: BigUint<M>,
}

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getCreatorToken)]
    #[storage_mapper("creatorToken")]
    fn creator_token(&self, user: &ManagedAddress) -> SingleValueMapper<TokenIdentifier>;

    #[view(getCreatorNft)]
    #[storage_mapper("creatorNft")]
    fn creator_nft(&self, user: &ManagedAddress) -> SingleValueMapper<TokenIdentifier>;

    #[view(getPaymentToken)]
    #[storage_mapper("paymentToken")]
    fn payment_token(&self, token: &TokenIdentifier) -> SingleValueMapper<TokenIdentifier>;

    #[view(getNftPrices)]
    #[storage_mapper("nftPrices")]
    fn nft_prices(&self, token: &TokenIdentifier) -> VecMapper<BigUint>;

    #[view(getCampaigns)]
    #[storage_mapper("campaigns")]
    fn campaigns(&self, user: &ManagedAddress) -> SingleValueMapper<Campaign<Self::Api>>;
}
