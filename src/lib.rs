extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(All)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    todo!()
}
