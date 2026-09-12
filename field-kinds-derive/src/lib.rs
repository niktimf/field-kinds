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
    let generics = &input.generics;
    let rename_all = parse::parse_rename_all(&input.attrs)?;
    let fields = parse::parse_fields(input)?;
    let crate_path = resolve_crate_path();

    Ok(generate::generate_all(
        struct_name,
        generics,
        &fields,
        rename_all,
        &crate_path,
    ))
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

#[cfg(test)]
mod tests {
    use super::{derive_impl, resolve_crate_path};

    /// The rejections a user can hit. They are unreachable from an
    /// integration test, since a `derive` that reports an error stops the
    /// test crate from compiling at all.
    fn error_for(input: &str) -> String {
        let input = syn::parse_str(input).expect("input should parse");
        match derive_impl(&input) {
            Ok(_) => panic!("deriving should have been rejected"),
            Err(error) => error.to_string(),
        }
    }

    fn accepts(input: &str) -> bool {
        let input = syn::parse_str(input).expect("input should parse");
        derive_impl(&input).is_ok()
    }

    #[test]
    fn tuple_structs_are_rejected() {
        assert_eq!(
            error_for("struct Point(u32, u32);"),
            "FieldKinds only supports structs with named fields"
        );
    }

    #[test]
    fn unit_structs_are_rejected() {
        assert_eq!(
            error_for("struct Marker;"),
            "FieldKinds only supports structs with named fields"
        );
    }

    #[test]
    fn enums_are_rejected() {
        assert_eq!(
            error_for("enum Shape { Circle, Square }"),
            "FieldKinds only supports structs"
        );
    }

    #[test]
    fn unions_are_rejected() {
        assert_eq!(
            error_for("union Slot { a: u32, b: f32 }"),
            "FieldKinds only supports structs"
        );
    }

    /// Dropping the rule would leave the fields under their original names,
    /// and the mismatch would only surface in the serialized output.
    #[test]
    fn unknown_rename_all_rule_is_rejected() {
        let message = error_for(
            r#"#[serde(rename_all = "bogusCase")] struct S { a: u32 }"#,
        );
        assert!(
            message.contains("bogusCase"),
            "the error should name the offending rule, got: {message}"
        );
    }

    /// `field_kinds` is this crate's own attribute, so `skipp` is a typo
    /// that must not pass silently.
    #[test]
    fn unknown_field_kinds_option_is_rejected() {
        let message = error_for("struct S { #[field_kinds(skipp)] a: u32 }");
        assert!(
            message.contains("field_kinds"),
            "the error should mention the attribute, got: {message}"
        );
    }

    #[test]
    fn known_rename_all_rules_are_accepted() {
        assert!(accepts(
            r#"#[serde(rename_all = "camelCase")] struct S { a: u32 }"#
        ));
    }

    #[test]
    fn field_kinds_options_are_accepted() {
        assert!(accepts("struct S { #[field_kinds(skip)] a: u32 }"));
        assert!(accepts("struct S { #[field_kinds(category = Text)] a: u32 }"));
    }

    #[test]
    fn named_structs_are_accepted() {
        assert!(accepts("struct User { id: u64, name: String }"));
    }

    /// Falls back to `::field_kinds` when the crate is not a dependency of
    /// the manifest being compiled, which is the case for this very crate.
    #[test]
    fn crate_path_falls_back_when_not_a_dependency() {
        assert_eq!(resolve_crate_path().to_string(), ":: field_kinds");
    }
}
