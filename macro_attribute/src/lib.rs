
extern crate proc_macro;

use proc_macro::{ TokenStream};


#[proc_macro_derive(MyDerive)]
pub fn my_derive_macro(input: TokenStream) -> TokenStream {
    // 在这里处理input...
    let mut string = input.to_string();
    string.push_str("//this is commentary !");
    string.parse().unwrap()
}