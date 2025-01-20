use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
pub struct QuickDefaultMarker {
    pub ident: syn::Ident,
}
impl Parse for QuickDefaultMarker {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().and_then(|ident: syn::Ident| {
            if ident != "quick_default" {
                return Err(syn::Error::new(ident.span(), "Expected quick-default"));
            }
            Ok(QuickDefaultMarker { ident })
        })
    }
}
#[derive(Clone)]
pub struct QuickDefaultAttribute {
    pub pound_token: syn::Token![#],
    pub style: syn::AttrStyle,
    pub bracket_token: syn::token::Bracket,
    pub meta: syn::Meta,
    pub attr_expr: syn::Lit,
}
impl TryFrom<syn::Attribute> for QuickDefaultAttribute {
    type Error = syn::Error;
    fn try_from(
        syn::Attribute {
            pound_token,
            style,
            bracket_token,
            meta,
        }: syn::Attribute,
    ) -> Result<Self, Self::Error> {
        let list = meta.require_list()?.clone();
        let ident = list.path.require_ident()?;

        if ident != "default" {
            return Err(syn::Error::new(ident.span(), "Malformed default"));
        };

        Ok(QuickDefaultAttribute {
            pound_token,
            style,
            bracket_token,
            meta,
            attr_expr: list.parse_args()?,
        })
    }
}
#[derive(Clone)]
pub struct QuickDefaultExpression(pub syn::Expr);
impl Default for QuickDefaultExpression {
    fn default() -> Self {
        QuickDefaultExpression(syn::parse_quote!(::std::default::Default::default()))
    }
}
impl Parse for QuickDefaultExpression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(QuickDefaultExpression)
    }
}
pub struct QuickDefaultField2 {
    pub field: syn::Field,
    pub def_expr: Option<QuickDefaultExpression>,
}
fn try_extract_field_default_attribute(
    ref in_field @ syn::Field {
        attrs: ref in_attrs,
        ..
    }: syn::Field,
) -> syn::Result<(syn::Field, Option<QuickDefaultExpression>)> {
    let mut def_attr = None;
    // Early-return, default is implied
    if in_attrs.is_empty() {
        return Ok((in_field.clone(), def_attr));
    }
    let mut out_attrs = vec![];
    for attr @ syn::Attribute {
        meta,
        pound_token,
        style,
        bracket_token,
    } in in_attrs
    {
        // If we cannot get a meta-list, skip this attribute
        let Ok(list) = meta.require_list() else {
            out_attrs.push(attr.clone());
            continue;
        };
        // If we cannot get an ident to match, skip this attribute
        let Ok(attr_ident) = list.path.require_ident() else {
            out_attrs.push(attr.clone());
            continue;
        };
        // Not the attribute we are looking for
        if attr_ident != "default" {
            out_attrs.push(attr.clone());
            continue;
        }

        // Here we propagate errors, malformed input is our responsibility
        let parse_res = list.parse_args()?;
        def_attr = Some(parse_res);
    }
    let out_field = syn::Field {
        attrs: out_attrs,
        ..in_field.clone()
    };
    Ok((out_field, def_attr))
}
pub struct QuickDefaultFieldsUnnamed2 {
    pub paren_token: syn::token::Paren,
    pub unnamed: syn::punctuated::Punctuated<QuickDefaultField2, syn::Token![,]>,
}
fn try_convert_fields_unnamed(
    syn::FieldsUnnamed {
        paren_token,
        unnamed,
    }: syn::FieldsUnnamed,
) -> syn::Result<QuickDefaultFieldsUnnamed2> {
    todo!()
}
fn unnamed_default_fields_to_default_body(
    QuickDefaultFieldsUnnamed2 {
        paren_token,
        unnamed,
    }: QuickDefaultFieldsUnnamed2,
) -> syn::punctuated::Punctuated<syn::FieldValue, syn::Token![,]> {
    let mut buf = syn::punctuated::Punctuated::new();
    for (index, pair) in unnamed.into_pairs().enumerate() {
        let value = pair.value();
        let member = syn::Member::from(index);
        let QuickDefaultExpression(expr) = value.def_expr.clone().unwrap_or_default();
        let field_value = syn::FieldValue {
            attrs: value.field.attrs.clone(),
            member,
            colon_token: value.field.colon_token,
            expr,
        };
        buf.push(field_value);
    }
    buf
}
pub struct QuickDefaultFieldsNamed2 {
    pub brace_token: syn::token::Brace,
    pub named: syn::punctuated::Punctuated<QuickDefaultField2, syn::Token![,]>,
}
fn named_default_fields_to_default_body(
    QuickDefaultFieldsNamed2 { brace_token, named }: QuickDefaultFieldsNamed2,
) -> syn::punctuated::Punctuated<syn::FieldValue, syn::Token![,]> {
    let mut buf = syn::punctuated::Punctuated::new();
    for (index, pair) in named.into_pairs().enumerate() {
        let value = pair.value();
        let ident = value.field.ident.clone().expect("Named field");
        let member = syn::Member::Named(ident);

        let QuickDefaultExpression(expr) = value.def_expr.clone().unwrap_or_default();
        let field_value = syn::FieldValue {
            attrs: value.field.attrs.clone(),
            member,
            colon_token: value.field.colon_token,
            expr,
        };
        buf.push(field_value);
    }
    buf
}
#[derive(Clone)]
pub struct QuickDefaultField {
    pub attrs: Vec<syn::Attribute>,
    pub default_value_attr: Option<QuickDefaultAttribute>,
    pub vis: syn::Visibility,
    pub mutability: syn::FieldMutability,

    /// Name of the field, if any.
    ///
    /// Fields of tuple structs have no names.
    pub ident: Option<syn::Ident>,
    pub colon_token: Option<syn::Token![:]>,
    pub ty: syn::Type,
}
impl From<QuickDefaultField> for syn::Field {
    fn from(value: QuickDefaultField) -> Self {
        syn::Field {
            attrs: value.attrs,
            vis: value.vis,
            mutability: value.mutability,
            ident: value.ident,
            colon_token: value.colon_token,
            ty: value.ty,
        }
    }
}
impl TryFrom<syn::Field> for QuickDefaultField {
    type Error = syn::Error;
    fn try_from(value: syn::Field) -> syn::Result<Self> {
        let syn::Field {
            attrs: field_attrs,
            vis,
            mutability,
            ident,
            colon_token,
            ty,
        } = value;
        // Create new attrs, so we remove our default attr
        let mut attrs = vec![];
        let mut default_value_attr: Option<QuickDefaultAttribute> = None;
        for attr in field_attrs {
            let meta = &attr.meta;
            let list = meta.require_list()?;
            if list.path.require_ident()? == "default" {
                if let Ok(default_attr) = attr.try_into() {
                    default_value_attr = Some(default_attr);
                }
            } else {
                attrs.push(attr);
            }
        }

        Ok(QuickDefaultField {
            attrs,
            default_value_attr,
            vis,
            mutability,
            ident,
            colon_token,
            ty,
        })
    }
}
impl ToTokens for QuickDefaultField {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let QuickDefaultField {
            default_value_attr,
            ident,
            colon_token,
            ..
        } = self;
        tokens.extend(quote! { #ident #colon_token });
        if let Some(QuickDefaultAttribute {
            attr_expr: value, ..
        }) = default_value_attr
        {
            tokens.extend(quote! { #value });
        } else {
            tokens.extend(quote! { ::std::default::Default::default() });
        }
    }
}
#[derive(Clone)]
pub struct QuickDefaultFieldsNamed {
    brace_token: syn::token::Brace,
    named: syn::punctuated::Punctuated<QuickDefaultField, syn::Token![,]>,
}
impl TryFrom<syn::FieldsNamed> for QuickDefaultFieldsNamed {
    type Error = syn::Error;
    fn try_from(
        syn::FieldsNamed { brace_token, named }: syn::FieldsNamed,
    ) -> Result<Self, Self::Error> {
        let mut output = syn::punctuated::Punctuated::new();
        for pair in named.into_pairs() {
            let value = match pair {
                syn::punctuated::Pair::Punctuated(field, _) => field.try_into()?,
                syn::punctuated::Pair::End(field) => field.try_into()?,
            };
            output.push(value);
        }
        Ok(QuickDefaultFieldsNamed {
            brace_token,
            named: output,
        })
    }
}
impl From<QuickDefaultFieldsNamed> for syn::FieldsNamed {
    fn from(QuickDefaultFieldsNamed { brace_token, named }: QuickDefaultFieldsNamed) -> Self {
        let mut output = syn::punctuated::Punctuated::new();
        for pair in named.into_pairs() {
            let field: syn::Field = pair.value().clone().into();
            output.push(field);
        }
        syn::FieldsNamed {
            brace_token,
            named: output,
        }
    }
}
#[derive(Clone)]
pub struct QuickDefaultFieldsUnnamed {
    paren_token: syn::token::Paren,
    unnamed: syn::punctuated::Punctuated<QuickDefaultField, syn::Token![,]>,
}
impl TryFrom<syn::FieldsUnnamed> for QuickDefaultFieldsUnnamed {
    type Error = syn::Error;
    fn try_from(
        syn::FieldsUnnamed {
            paren_token,
            unnamed,
        }: syn::FieldsUnnamed,
    ) -> Result<Self, Self::Error> {
        let mut output = syn::punctuated::Punctuated::new();
        for pair in unnamed.into_pairs() {
            let value = match pair {
                syn::punctuated::Pair::Punctuated(field, _) => field.try_into()?,
                syn::punctuated::Pair::End(field) => field.try_into()?,
            };
            output.push(value);
        }
        Ok(QuickDefaultFieldsUnnamed {
            paren_token,
            unnamed: output,
        })
    }
}
impl From<QuickDefaultFieldsUnnamed> for syn::FieldsUnnamed {
    fn from(
        QuickDefaultFieldsUnnamed {
            paren_token,
            unnamed,
        }: QuickDefaultFieldsUnnamed,
    ) -> Self {
        let mut output = syn::punctuated::Punctuated::new();
        for pair in unnamed.into_pairs() {
            let field: syn::Field = pair.value().clone().into();
            output.push(field);
        }
        syn::FieldsUnnamed {
            paren_token,
            unnamed: output,
        }
    }
}
#[derive(Clone)]
pub enum QuickDefaultFields {
    DefaultNamed(QuickDefaultFieldsNamed),
    DefaultUnnamed(QuickDefaultFieldsUnnamed),
    SynFields(syn::Fields),
}

impl From<syn::Fields> for QuickDefaultFields {
    fn from(value: syn::Fields) -> Self {
        match value {
            ref named @ syn::Fields::Named(ref fields) => {
                QuickDefaultFieldsNamed::try_from(fields.clone())
                    .map(QuickDefaultFields::DefaultNamed)
                    .unwrap_or(QuickDefaultFields::SynFields(named.clone()))
            }

            ref unnamed @ syn::Fields::Unnamed(ref fields) => {
                QuickDefaultFieldsUnnamed::try_from(fields.clone())
                    .map(QuickDefaultFields::DefaultUnnamed)
                    .unwrap_or(QuickDefaultFields::SynFields(unnamed.clone()))
            }
            unit @ syn::Fields::Unit => QuickDefaultFields::SynFields(unit),
        }
    }
}
impl From<QuickDefaultFields> for syn::Fields {
    fn from(value: QuickDefaultFields) -> syn::Fields {
        match value {
            QuickDefaultFields::DefaultNamed(named) => syn::Fields::Named(named.into()),
            QuickDefaultFields::DefaultUnnamed(unnamed) => syn::Fields::Unnamed(unnamed.into()),
            QuickDefaultFields::SynFields(fields) => fields,
        }
    }
}
impl ToTokens for QuickDefaultFields {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let c = self.clone();
        let fields: syn::Fields = c.into();
        fields.to_tokens(tokens);
    }
}
#[derive(Clone)]
pub struct QuickDefaultStruct {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub struct_token: syn::Token![struct],
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub fields: QuickDefaultFields,
    pub semi_token: Option<syn::Token![;]>,
}
impl From<syn::ItemStruct> for QuickDefaultStruct {
    fn from(value: syn::ItemStruct) -> Self {
        let syn::ItemStruct {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            fields,
            semi_token,
        } = value;
        let fields: QuickDefaultFields = fields.into();
        QuickDefaultStruct {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            fields,
            semi_token,
        }
    }
}
impl From<QuickDefaultStruct> for syn::ItemStruct {
    fn from(value: QuickDefaultStruct) -> Self {
        let QuickDefaultStruct {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            fields,
            semi_token,
        } = value;
        let fields = fields.into();
        syn::ItemStruct {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            fields,
            semi_token,
        }
    }
}
impl Parse for QuickDefaultStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(syn::ItemStruct::into)
    }
}
impl ToTokens for QuickDefaultStruct {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let c = self.clone();
        let r#struct: syn::ItemStruct = c.into();
        r#struct.to_tokens(tokens);

        let c = self.clone();
        let r#impl: QuickDefaultImplementation = c.into();
        r#impl.to_tokens(tokens);
    }
}
pub struct QuickDefaultImplementation {
    pub implementor_ident: syn::Ident,
    pub implementor_fields: QuickDefaultFields,
}
impl From<QuickDefaultStruct> for QuickDefaultImplementation {
    fn from(QuickDefaultStruct { ident, fields, .. }: QuickDefaultStruct) -> Self {
        QuickDefaultImplementation {
            implementor_ident: ident,
            implementor_fields: fields,
        }
    }
}
impl ToTokens for QuickDefaultImplementation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut default_body_buf = quote! {};
        match self.implementor_fields {
            QuickDefaultFields::DefaultNamed(QuickDefaultFieldsNamed { ref named, .. }) => {
                let mut default_body_content = quote! {};
                for field in named.clone().into_pairs() {
                    default_body_content.extend(quote! { #field });
                }
                default_body_buf.extend(quote! {
                    {
                        #default_body_content
                    }
                });
            }
            QuickDefaultFields::DefaultUnnamed(QuickDefaultFieldsUnnamed {
                ref unnamed, ..
            }) => {
                let mut default_body_content = quote! {};
                for field in unnamed.clone().into_pairs() {
                    default_body_content.extend(quote! { #field });
                }
                default_body_buf.extend(quote! {
                    (
                        #default_body_content
                    )
                });
            }
            QuickDefaultFields::SynFields(syn::Fields::Named(syn::FieldsNamed {
                ref named,
                ..
            })) => {
                let mut default_body_content = quote! {};
                for field in named.clone().into_pairs() {
                    let ident = field.value().ident.as_ref().unwrap();
                    default_body_content.extend(quote! {#ident: });
                    default_body_content.extend(match field {
                        syn::punctuated::Pair::End(_) => {
                            quote! { ::std::default::Default::default() }
                        }
                        syn::punctuated::Pair::Punctuated(_, punct) => {
                            quote! {::std::default::Default::default(), #punct }
                        }
                    })
                }
                default_body_buf.extend(quote! {
                    {
                        #default_body_content
                    }
                });
            }
            QuickDefaultFields::SynFields(syn::Fields::Unnamed(syn::FieldsUnnamed {
                ref unnamed,
                ..
            })) => {
                let mut default_body_content = quote! {};
                for field in unnamed.clone().into_pairs() {
                    default_body_content.extend(match field {
                        syn::punctuated::Pair::End(_) => {
                            quote! { ::std::default::Default::default() }
                        }
                        syn::punctuated::Pair::Punctuated(_, punct) => {
                            quote! {::std::default::Default::default(), #punct }
                        }
                    })
                }
                default_body_buf.extend(quote! {
                    (
                        #default_body_content
                    )
                });
            }
            _ => {}
        }
        let implementor_ident = &self.implementor_ident;
        tokens.extend(quote! {
            impl Default for #implementor_ident {
                fn default() -> Self {
                    #implementor_ident #default_body_buf
                }
            }
        });
    }
}
pub fn quick_default2(_: TokenStream2, input: TokenStream2) -> TokenStream2 {
    let r#struct: syn::Result<QuickDefaultStruct> = syn::parse2(input);
    if let Err(e) = r#struct {
        return e.into_compile_error();
    }
    let r#struct = unsafe { r#struct.unwrap_unchecked() };
    quote! { #r#struct }
}
