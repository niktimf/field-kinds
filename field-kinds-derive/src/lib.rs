mod field;
mod generate;
mod parse;

use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use quote::{format_ident, quote};
use syn::parse_macro_input;

#[proc_macro_derive(FieldKinds, attributes(field_kinds, field_tags, serde))]
pub fn derive_field_kinds(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    match derive_impl(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn derive_impl(
    input: &syn::DeriveInput,
) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name = &input.ident;
    let rename_all = parse::parse_rename_all(&input.attrs);
    let fields = parse::parse_fields(input)?;
    let crate_path = resolve_crate_path();

    Ok(generate::generate_all(struct_name, &fields, rename_all, &crate_path))
}

fn resolve_crate_path() -> proc_macro2::TokenStream {
    match crate_name("field-kinds") {
        Ok(FoundCrate::Name(name)) => {
            let ident = format_ident!("{}", name);
            quote! { ::#ident }
        }
        Ok(FoundCrate::Itself) | Err(_) => quote! { ::field_kinds },
    }
}
