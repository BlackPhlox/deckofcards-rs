use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse::{Parse, ParseStream}, Error, MetaList};

struct DeckyParams(syn::Ident);

impl Parse for DeckyParams {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        //let content;
        //syn::parenthesized!(content in input);
        let card_type = input.parse()?;
        Ok(DeckyParams(card_type))
    }
}

pub fn decky_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_decky_macro(&ast)
}

fn impl_decky_macro(ast: &syn::DeriveInput) -> TokenStream {
    let _name = &ast.ident;
    let attribute = ast.attrs.iter().filter(
        |a| a.path().segments.len() == 1 && a.path().segments[0].ident == "decky_cards"
    ).nth(0).expect("decky_cards attribute required for deriving Decky!");
    let tokens = match attribute.meta.clone() {
        syn::Meta::List(MetaList { path: _, delimiter: _, tokens: ts }) => ts,
        _ => panic!("Invalid attribute"),
    }; 
    let parameters: DeckyParams = syn::parse2(tokens).expect("Invalid decky_cards attribute!");
    let _card_type = parameters.0;

    let gen = quote!{};
    gen.into()
}