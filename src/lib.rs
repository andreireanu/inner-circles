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
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
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
        // callback is called with ESDTTransfer of the newly issued token, with the amount requested,
        // so we can get the token identifier and amount from the call data
        // TO DO
        // self.user_token(&caller).set(&token_identifier);
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
}
