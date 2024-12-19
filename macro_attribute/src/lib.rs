extern crate proc_macro;

use proc_macro::{ TokenStream};


#[proc_macro_derive(MyDerive)]
pub fn my_derive_macro(input: TokenStream) -> TokenStream {
    input
}
