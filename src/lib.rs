#![no_std]
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod storage;
 
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
                self.creator_token(caller).set(token_identifier.unwrap_esdt());
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
    #[endpoint(issueSemiFungibleToken)]
    fn issue_semi_fungible_token(
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
                self.creator_sft(caller).set(&token_identifier);
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

    #[only_owner]
    #[endpoint(clear)]
    fn clear(&self, address: &ManagedAddress) {
        self.creator_token(address).clear();
    }

    #[endpoint(getEsdtTokenData)]
    fn get_esdt_token_data(&self, address: &ManagedAddress) -> EsdtTokenData<Self::Api> {
        let token = self.creator_token(address).get();
        let sc_address = self.blockchain().get_sc_address();
        self.blockchain()
            .get_esdt_token_data(&sc_address, &token, 0)
    }
}
