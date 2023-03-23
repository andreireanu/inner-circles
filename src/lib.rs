#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait InnerCircles {
    #[init]
    fn init(&self) {}
}

// fn issue(token_type: EsdtTokenType, issue_cost: BigUint, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer,initial_supply: BigUint, num_decimals: usize, opt_callback: Option<CallbackClosure>) -> !
