#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod storage;

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct ExampleAttributes2<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct ExampleAttributes {
    pub creation_timestamp: u64,
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
        let issue_cost = self.call_value().egld_value();
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
                self.user_token(caller).set(token_identifier.unwrap_esdt());
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
    // Claim fungible token
    #[endpoint(claimToken)]
    fn claim_token(&self, amount: &BigUint, token: &TokenIdentifier) {
        let caller = self.blockchain().get_caller();
        self.send().direct_esdt(&caller, &token, 0, amount);
    }



    ////////////////
    // Issue semi fungible token
    #[endpoint(issueSemiFungibleToken)]
    fn issue_semi_fungible_token(
        &self,
        token_display_name: &ManagedBuffer,
        token_ticker: &ManagedBuffer,
    ) {
        let issue_cost = self.call_value().egld_value();
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
            .with_callback(self.callbacks().sft_issue_callback(&caller))
            .call_and_exit()
    }

    #[callback]
    fn sft_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_identifier) => {
                self.user_sft(caller).set(&token_identifier);
                self.set_local_roles_sft(&token_identifier);
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
    fn set_local_roles_sft(&self, sft_token: &TokenIdentifier) {
        let sc_address = self.blockchain().get_sc_address();
        let roles = [
            EsdtLocalRole::NftCreate,
            EsdtLocalRole::NftAddQuantity,
            EsdtLocalRole::NftBurn,
        ];
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(&sc_address, sft_token, roles[..].iter().cloned())
            .async_call()
            .call_and_exit()
    }

    #[endpoint(createSft)]
    #[allow(clippy::too_many_arguments)]
    fn create_sft_with_attributes(
        &self,
        name: ManagedBuffer,
        uri: ManagedBuffer,
        attribute_new: ManagedBuffer,
    ) {
        let caller = self.blockchain().get_caller();
        let sft_token_id = &self.user_sft(&caller).get();

        let attributes = ExampleAttributes2 {
            name: attribute_new,
        };

        let mut serialized_attributes = ManagedBuffer::new();
        if let core::result::Result::Err(err) = attributes.top_encode(&mut serialized_attributes) {
            sc_panic!("Attributes encode error: {}", err.message_bytes());
        }

        let attributes_sha256 = self.crypto().sha256(&serialized_attributes);
        let attributes_hash = attributes_sha256.as_managed_buffer();

        let uris = ManagedVec::from_single_item(uri);

        let _ = self.send().esdt_nft_create(
            &sft_token_id,
            &BigUint::from(20u64),
            &name,
            &BigUint::from(0u64),
            attributes_hash,
            &attributes,
            &uris,
        );
    }

    ////////////////
    // Issue non fungible token
    #[payable("EGLD")]
    #[endpoint(issueNonFungibleToken)]
    fn issue_non_fungible_token(
        &self,
        token_display_name: &ManagedBuffer,
        token_ticker: &ManagedBuffer,
    ) {
        let issue_cost = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();
        self.send()
            .esdt_system_sc_proxy()
            .issue_non_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                NonFungibleTokenProperties {
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
                self.user_nft(caller).set(&token_identifier);
                self.set_local_roles_nft(&token_identifier);
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
    fn set_local_roles_nft(&self, nft_token: &TokenIdentifier) {
        let sc_address = self.blockchain().get_sc_address();
        let roles = [
            EsdtLocalRole::NftCreate,
            EsdtLocalRole::NftBurn,
            EsdtLocalRole::NftAddUri,
            EsdtLocalRole::NftUpdateAttributes,
        ];
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(&sc_address, nft_token, roles[..].iter().cloned())
            .async_call()
            .call_and_exit()
    }

    #[endpoint(createNft)]
    fn create_nft_with_attributes(
        &self,
        name: ManagedBuffer,
        uri: ManagedBuffer,
        attribute_new: ManagedBuffer,
    ) {
        let caller = self.blockchain().get_caller();
        let nft_token_id = &self.user_nft(&caller).get();

        let attributes = ExampleAttributes2 {
            name: attribute_new,
        };

        let mut serialized_attributes = ManagedBuffer::new();
        if let core::result::Result::Err(err) = attributes.top_encode(&mut serialized_attributes) {
            sc_panic!("Attributes encode error: {}", err.message_bytes());
        }

        let attributes_sha256 = self.crypto().sha256(&serialized_attributes);
        let attributes_hash = attributes_sha256.as_managed_buffer();

        let uris = ManagedVec::from_single_item(uri);

        let _ = self.send().esdt_nft_create(
            &nft_token_id,
            &BigUint::from(20u64),
            &name,
            &BigUint::from(0u64),
            attributes_hash,
            &attributes,
            &uris,
        );
    }

    ////////////////
    // Buy
    #[endpoint(buySft)]
    #[payable("*")]
    fn buy_sft(&self, address: ManagedAddress, token_sft_nonce: u64) {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();
        require!(
            self.user_token(&address).get() == payment.token_identifier,
            "Wrong token"
        );
        let user_sft = self.user_sft(&address);
        require!(!user_sft.is_empty(), "Empty");
        let user_nft_token = user_sft.get();
        self.send().esdt_local_burn(
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );
        self.send().direct_esdt(
            &caller,
            &user_nft_token,
            token_sft_nonce,
            &BigUint::from(1u64),
        );
    }
}
