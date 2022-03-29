#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

// Change the token ID here
const TOKEN_ID: &[u8; 11] = b"GTTS-ac8274";

#[elrond_wasm::contract]
pub trait SmartContract {
    #[init]
    fn init(&self) {}

    #[payable("EGLD")]
    #[endpoint(swap)]
    fn swap(
        &self,
        #[payment_amount] amount: BigUint
    ) -> SCResult<()> {
        require!(amount != 0, "No");

        // This sets the price for the token, here 1 egld = 1000 in the token
        let token_price = BigUint::from(1000_u32);
        let quantity = amount * token_price;
        let caller = self.blockchain().get_caller();
        let identifier = TokenIdentifier::from(ManagedBuffer::new_from_bytes(TOKEN_ID));

        Self::Api::send_api_impl().direct_esdt_execute(
            &caller,
            &identifier,
            &quantity,
            50_000_000,
            &ManagedBuffer::new_from_bytes(b"ESDTTransfer"),
            &ManagedArgBuffer::new_empty()
        ).into()
    }
}
