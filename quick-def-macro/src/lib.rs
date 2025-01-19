extern crate proc_macro;

use darling::{FromDeriveInput, FromField, FromMeta, ast};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::parse::{Parse, ParseStream};

/// Support parsing a Deref::Target from a field
#[derive(Debug, FromField)]
// We want to process all attributes declared with `deref_target`
#[darling(attributes(deref_target))]
struct DerefTargetReceiver {
    /// The field's ident, or none for tuple
    ident: Option<syn::Ident>,
    vis: syn::Visibility,
    ty: syn::Type,
    attrs: Vec<syn::Attribute>,
}
#[derive(Debug, FromMeta)]
struct DerefTargetFlag {
    deref_target: darling::util::Flag,
    ident: syn::Ident,
    ty: syn::Type,
}
#[derive(Debug, FromDeriveInput)]
#[darling(supports(struct_any))]
struct QuickDerefReceiver {
    /// Ident,
    ident: syn::Ident,
    deref_target: DerefTargetFlag,
}
#[proc_macro_derive(QuickDeref)]
pub fn quick_deref(input: TokenStream) -> TokenStream {
    todo!()
}

/// Support parsing
#[derive(Debug)]
struct QuickDefReceiver {
    meta: syn::Meta,
    item: syn::ItemStruct,
}
impl Parse for QuickDefReceiver {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let meta = input.parse()?;
        let item = input.parse()?;
        Ok(QuickDefReceiver { meta, item })
    }
}
impl ToTokens for QuickDefReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let QuickDefReceiver {
            meta,
            item:
                item @ syn::ItemStruct {
                    attrs,
                    vis,
                    struct_token,
                    ident: quick_def,
                    generics,
                    fields,
                    semi_token,
                },
        } = self;
        tokens.extend(quote! { #item });
        let mut field_idx = 0;
        let mut deref_target = false;
        for field @ syn::Field {
            attrs,
            vis,
            mutability,
            ident: field_ident,
            colon_token: _,
            ty,
        } in fields
        {
            if !deref_target {
                if let Ok(DerefTargetReceiver {
                    ident,
                    vis,
                    ty: target,
                    attrs,
                }) = DerefTargetReceiver::from_field(field)
                {
                    let ident = ident
                        .map(|val| quote! { #val })
                        .unwrap_or_else(|| quote! { #field_idx });
                    tokens.extend(quote! {
                        impl ::core::ops::Deref for #quick_def {
                            type Target = #target;
                            fn deref(&self) -> &Self::Target {
                                &self.#ident
                            }
                        }
                    });
                    deref_target = true;
                }
            }
            field_idx += 1;
        }
    }
}

#[proc_macro_attribute]
pub fn quick_def(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match ast::NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let x = syn::parse_macro_input!(input as QuickDefReceiver);
    // println!("Receiver: {:?}", x);
    quote! { #x }.into()
}
