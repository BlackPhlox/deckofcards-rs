use proc_macro::TokenStream;

extern crate proc_macro;

mod handy;
mod decky;

#[proc_macro_derive(Handy, attributes(handy_cards))]
pub fn handy_derive(input: TokenStream) -> TokenStream {
    handy::handy_derive(input)
}

#[proc_macro_derive(Decky, attributes(handy_cards))]
pub fn decky_derive(input: TokenStream) -> TokenStream {
    decky::decky_derive(input)
}