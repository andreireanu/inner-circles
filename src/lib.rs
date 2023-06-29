#![no_std]

use storage::Campaign;
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod storage;

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct NftAttributes<M: ManagedTypeApi> {
    pub attribute1: ManagedBuffer<M>,
}

#[multiversx_sc::contract]
pub trait InnerCircles: crate::storage::StorageModule {
    #[init]
    fn init(&self) {}

    ////////////////
    // Issue fungible token
    #[payable("EGLD")]
    #[endpoint(issueFungibleToken)]
    fn issue_fungible_token(
        &self,
        token_display_name: &ManagedBuffer,
        token_ticker: &ManagedBuffer,
        initial_supply: &BigUint,
    ) {
        let issue_cost = self.call_value().egld_value().clone_value();
        let caller = self.blockchain().get_caller();

        self.send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: 0,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().issue_fungible_callback(&caller))
            .call_and_exit()
    }

    #[callback]
    fn issue_fungible_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let (token_identifier, returned_tokens) = self.call_value().egld_or_single_fungible_esdt();
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.creator_token(caller)
                    .set(token_identifier.unwrap_esdt());
            }
            ManagedAsyncCallResult::Err(_message) => {
                // return issue cost to the caller
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens);
                }
            }
        }
    }

    ////////////////
    // Issue semi fungible token
    #[payable("EGLD")]
    #[endpoint(issueNonFungibleToken)]
    fn issue_non_fungible_token(
        &self,
        token_display_name: &ManagedBuffer,
        token_ticker: &ManagedBuffer,
    ) {
        let issue_cost = self.call_value().egld_value().clone_value();
        let caller = self.blockchain().get_caller();
        self.send()
            .esdt_system_sc_proxy()
            .issue_semi_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                SemiFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_transfer_create_role: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().nft_issue_callback(&caller))
            .call_and_exit()
    }

    #[callback]
    fn nft_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_identifier) => {
                self.creator_nft(caller).set(&token_identifier);
                let payment_tkn = self.creator_token(&caller).get();
                self.payment_token(&token_identifier).set(&payment_tkn);
                self.set_local_roles(&token_identifier);
            }
            ManagedAsyncCallResult::Err(_message) => {
                // return issue cost to the caller
                let (token_identifier, returned_tokens) =
                    self.call_value().egld_or_single_fungible_esdt();
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens);
                }
            }
        }
    }

    ////////////////
    // Set minting roles
    #[inline]
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self, token_identifier: &TokenIdentifier) {
        let sc_address = self.blockchain().get_sc_address();
        let roles = [
            EsdtLocalRole::NftCreate,
            EsdtLocalRole::NftAddQuantity,
            EsdtLocalRole::NftBurn,
        ];
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(&sc_address, token_identifier, roles[..].iter().cloned())
            .async_call()
            .call_and_exit()
    }

    ////////////////
    // Create Nft
    #[endpoint(createNft)]
    fn create_nft_with_attributes(
        &self,
        name: ManagedBuffer,
        amount: BigUint,
        uri: ManagedBuffer,
        attribute: ManagedBuffer,
        price: BigUint,
    ) {
        let caller = self.blockchain().get_caller();
        let user_nft_mapper = self.creator_nft(&caller);
        require!(
            !user_nft_mapper.is_empty(),
            "User does not have a token issued"
        );

        let attributes = NftAttributes {
            attribute1: attribute,
        };

        let mut serialized_attributes = ManagedBuffer::new();
        if let core::result::Result::Err(err) = attributes.top_encode(&mut serialized_attributes) {
            sc_panic!("Attributes encode error: {}", err.message_bytes());
        }

        let attributes_sha256 = self.crypto().sha256(&serialized_attributes);
        let attributes_hash = attributes_sha256.as_managed_buffer();

        let uris = ManagedVec::from_single_item(uri);

        self.send().esdt_nft_create(
            &user_nft_mapper.get(), // Token name
            &amount,                // Amount to mint
            &name,                  // Nft display name
            &BigUint::from(0u64),   // Royalties
            attributes_hash,        // Nft Hash
            &attributes,            // Non formalized attributes
            &uris,                  // uris
        );
        self.nft_prices(&user_nft_mapper.get()).push(&price);
    }

    ////////////////
    // Create Campaign
    #[endpoint(createCampaign)]
    fn create_campaign(&self, name: ManagedBuffer, hashtag: ManagedBuffer, amount: BigUint) {
        let campaign = Campaign {
            name,
            hashtag,
            amount,
        };
        let caller = self.blockchain().get_caller();
        self.campaigns(&caller).set(&campaign);
    }

    ////////////////
    // Send Campaign Tokens
    #[endpoint(sendCampaignTokens)]
    fn send_campaign_tokens(
        &self,
        hashtag: ManagedBuffer,
        destinations: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>,
    ) {
        let caller = self.blockchain().get_caller();
        require!(
            self.campaigns(&caller).get().hashtag == hashtag,
            "Campaign not found"
        );
        let payment_token = self.creator_token(&caller);
        let esdt_payment_token = EgldOrEsdtTokenIdentifier::esdt(payment_token.get());
        require!(!payment_token.is_empty(), "Creator hasn't created a token");

        let mut amount_to_spend = BigUint::from(0u64);

        for destination in destinations.clone() {
            let (_address_to_send, amount_to_send) = destination.into_tuple();
            amount_to_spend += &amount_to_send;
        }

        let payment_amount = self.blockchain().get_sc_balance(&esdt_payment_token, 0);

        require!(
            payment_amount > amount_to_spend,
            "Not enough tokens left in Smart Contract"
        );

        for destination in destinations {
            let (address_to_send, amount_to_send) = destination.into_tuple();
            self.send()
                .direct(&address_to_send, &esdt_payment_token, 0, &amount_to_send);
        }
    }


    ////////////////
    // Buy
    #[endpoint(buyNft)]
    #[payable("*")]
    fn buy_nft(&self, nft_token: &TokenIdentifier, nft_token_nonce: u64) {
        let payment = self.call_value().single_esdt();
        let needed_payment_token = self.payment_token(&nft_token).get();
        require!(
            &needed_payment_token == &payment.token_identifier,
            "You tried to buy the NFT with the wrong error"
        );
        let nft_price = self.get_nft_price(&nft_token, nft_token_nonce.clone() as usize);
        require!(&payment.amount == &nft_price, "Payment amount incorrect");

        self.send().esdt_local_burn(
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );

        let caller = self.blockchain().get_caller();
        self.send()
            .direct_esdt(&caller, &nft_token, nft_token_nonce, &BigUint::from(1u64));
    }

    ////////////////
    // Get NFT price
    #[view(getNftPrice)]
    fn get_nft_price(&self, token: &TokenIdentifier, idx: usize) -> BigUint {
        self.nft_prices(token).get(idx)
    }    

    ////////////////
    // Clear storages
    #[only_owner]
    #[endpoint(clearToken)]
    fn clear_token(&self, address: &ManagedAddress) {
        self.creator_token(address).clear();
    }

    #[only_owner]
    #[endpoint(clearNft)]
    fn clear_nft(&self, address: &ManagedAddress) {
        self.creator_nft(address).clear();
    }

    #[only_owner]
    #[endpoint(clearCampaign)]
    fn clear_campaign(&self, address: &ManagedAddress) {
        self.campaigns(address).clear();
    }

    #[only_owner]
    #[endpoint(clearNftPrices)]
    fn clear_nft_prices(&self, token: &TokenIdentifier) {
        self.nft_prices(token).clear();
    }

    #[only_owner]
    #[endpoint(clearPaymentToken)]
    fn clear_payment_token(&self, token: &TokenIdentifier) {
        self.payment_token(token).clear();
    }
}
