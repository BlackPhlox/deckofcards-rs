use proc_macro::TokenStream;

extern crate proc_macro;

mod handy;

#[proc_macro_derive(Handy, attributes(handy_cards))]
pub fn handy_derive(input: TokenStream) -> TokenStream {
    handy::handy_derive(input)
}