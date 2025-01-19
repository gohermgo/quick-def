extern crate proc_macro;
use proc_macro::TokenStream;
#[proc_macro_attribute]
pub fn quick_default(args: TokenStream, input: TokenStream) -> TokenStream {
    quick_default_core::quick_default2(args.into(), input.into()).into()
}
