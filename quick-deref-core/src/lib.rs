use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

#[derive(Debug)]
pub struct DerefTargetMetaArgs {
    pub target_token: Option<syn::Ident>,
    pub equals_token: Option<syn::Token![=]>,
    pub target_ident: syn::Ident,
}
impl Parse for DerefTargetMetaArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let meta_args = if input.peek2(syn::Token![=]) {
            let target_token: syn::Ident = input.parse()?;
            if target_token != "target" {
                return Err(syn::Error::new(target_token.span(), "Invalid target token"));
            }
            let equals_token = input.parse()?;
            let target_ident = input.parse()?;
            DerefTargetMetaArgs {
                target_token: Some(target_token),
                equals_token: Some(equals_token),
                target_ident,
            }
        } else {
            let target_ident = input.parse()?;
            DerefTargetMetaArgs {
                target_token: None,
                equals_token: None,
                target_ident,
            }
        };

        Ok(meta_args)
    }
}

pub struct DerefImplementation {
    implementor: syn::ItemStruct,
    args: Option<DerefTargetMetaArgs>,
}
impl ToTokens for DerefImplementation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let DerefImplementation {
            ref implementor,
            ref args,
        } = *self;
        tokens.extend(quote! { #implementor });
        let syn::ItemStruct {
            attrs: _,
            vis: _,
            struct_token: _,
            ident: implementor_ident,
            generics: _,
            fields,
            semi_token: _,
        } = implementor;

        // Early return for unit structs
        let Some(DerefTargetMetaArgs {
            target_token: _,
            equals_token: _,
            target_ident,
        }) = args
        else {
            // Assume unit struct
            let Some(new_type_field) = fields.iter().next() else {
                tokens
                    .extend(syn::Error::new(fields.span(), "Bad unit-field").into_compile_error());
                return;
            };
            let ty = &new_type_field.ty;
            tokens.extend(quote! {
                impl ::core::ops::Deref for #implementor_ident {
                    type Target = #ty;
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }
            });
            return;
        };

        // Find the field among struct-fields
        for field in fields.iter() {
            let Some(field_ident) = field.ident.as_ref() else {
                continue;
            };

            if field_ident == target_ident {
                let ty = &field.ty;
                tokens.extend(quote! {
                    impl ::core::ops::Deref for #implementor_ident {
                        type Target = #ty;
                        fn deref(&self) -> &Self::Target {
                            &self.#field_ident
                        }
                    }
                });
            }
        }
    }
}
// Assume the new-type pattern is in use
fn quick_deref2_empty(input: TokenStream2) -> TokenStream2 {
    let recv: syn::Result<syn::ItemStruct> = syn::parse2(input);
    if let Err(e) = recv {
        return e.to_compile_error();
    }
    let implementation = unsafe {
        recv.map(|implementor| DerefImplementation {
            implementor,
            args: None,
        })
        .unwrap_unchecked()
    };
    quote! { #implementation }
}
fn quick_deref2_with_args(args: TokenStream2, input: TokenStream2) -> TokenStream2 {
    let target: syn::Result<DerefTargetMetaArgs> = syn::parse2(args);
    if let Err(e) = target {
        return e.to_compile_error();
    }
    let args = unsafe { target.unwrap_unchecked() };

    let recv: syn::Result<syn::ItemStruct> = syn::parse2(input);
    if let Err(e) = recv {
        return e.to_compile_error();
    }
    let implementation = unsafe {
        recv.map(|implementor| DerefImplementation {
            implementor,
            args: Some(args),
        })
        .unwrap_unchecked()
    };

    quote! { #implementation }
}
// I imagine like
// #[quick_deref(field)]
pub fn quick_deref2(args: TokenStream2, input: TokenStream2) -> TokenStream2 {
    if args.is_empty() {
        quick_deref2_empty(input)
    } else {
        quick_deref2_with_args(args, input)
    }
}
