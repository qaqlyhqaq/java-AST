#![feature(proc_macro_quote)]
extern crate proc_macro;

use proc_macro::{quote, TokenStream};
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(MyDerive)]
pub fn my_derive_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let generate = quote!{
        impl MyDerive for #input {

        }
    };
    generate.into()
}
