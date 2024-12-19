#[proc_macro_attribute]
pub fn my_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 在这里处理attr和item...
    item
}
