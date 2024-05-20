use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse::{Parse, ParseStream}, Error, MetaList};

struct MyParams(syn::Ident);

impl Parse for MyParams {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        //let content;
        //syn::parenthesized!(content in input);
        let card_type = input.parse()?;
        Ok(MyParams(card_type))
    }
}

pub fn handy_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_handy_macro(&ast)
}

fn impl_handy_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let attribute = ast.attrs.iter().filter(
        |a| a.path().segments.len() == 1 && a.path().segments[0].ident == "handy_cards"
    ).nth(0).expect("handy_cards attribute required for deriving Handy!");
    let tokens = match attribute.meta.clone() {
        syn::Meta::List(MetaList { path: _, delimiter: _, tokens: ts }) => ts,
        _ => panic!("Invalid attribute"),
    }; 
    let parameters: MyParams = syn::parse2(tokens).expect("Invalid handy_cards attribute!");
    let card_type = parameters.0;

    let gen = quote! {
        use deckofcards::{card, Cards, Hand, Handy};

        impl Default for #name {
            fn default() -> Self {
                Self(Area { cards: vec![] })
            }
        }

        impl Cards<#card_type> for #name {
            fn cards(&self) -> &[#card_type] {
                self.0.cards.as_slice()
            }
        
            fn mut_cards(&mut self) -> &mut [#card_type] {
                self.0.cards.as_mut_slice()
            }
        }

        impl Handy<#card_type> for #name {
            fn new() -> Self {
                Self::default()
            }
        
            fn from_hand(hand: &Self) -> Self {
                Self::from_cards(hand.cards())
            }
        
            fn from_cards(cards: &[#card_type]) -> Self {
                Self {
                    0: Area {
                        cards: Vec::from(cards),
                    },
                }
            }
        
            fn from_strings(card_slice: &[&str]) -> Self {
                todo!()
            }
        
            fn push_card(&mut self, card: #card_type) {
                self.0.cards.push(card);
            }
        
            fn push_cards(&mut self, cards: &[#card_type]) {
                //self.0.cards.extend(cards);
                todo!()
            }
        
            fn push_hand(&mut self, other: &Self) {
                //self.0.cards.extend(other.cards());
                todo!()
            }
        
            fn len(&self) -> usize {
                self.0.cards.len()
            }
        
            fn clear(&mut self) {
                self.0.cards.clear();
            }
        
            fn remove(&mut self, index: usize) -> #card_type {
                self.0.cards.remove(index)
            }
        
            fn remove_cards(&mut self, cards: &[#card_type]) {
                for c in cards {
                    let _ = self.remove_card(c);
                }
            }
        
            fn remove_all_cards(&mut self, cards: &[#card_type]) {
                for c in cards {
                    while self.remove_card(c) {}
                }
            }
        
            fn remove_card(&mut self, card: &#card_type) -> bool {
                if let Some(pos) = self.0.cards.iter().position(|c| c == card) {
                    let _ = self.0.cards.remove(pos);
                    true
                } else {
                    false
                }
            }
        }
    };
    gen.into()
}