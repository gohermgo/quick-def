extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn quick_deref(args: TokenStream, input: TokenStream) -> TokenStream {
    quick_deref_core::quick_deref2(args.into(), input.into()).into()
}
