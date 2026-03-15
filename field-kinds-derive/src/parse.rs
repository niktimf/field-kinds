use crate::field::{ParsedField, RenameRule};
use convert_case::Case;
use syn::{Attribute, DeriveInput, Field, Ident, Lit, LitStr, Result};

/// Parses `rename_all` from `#[serde(rename_all = "...")]`
pub fn parse_rename_all(attrs: &[Attribute]) -> Option<RenameRule<'_>> {
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        let mut result = None;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename_all") {
                let value: LitStr = meta.value()?.parse()?;
                result = string_to_rename_rule(&value.value());
            }
            Ok(())
        });

        if result.is_some() {
            return result;
        }
    }
    None
}

/// Parses all fields of the struct
pub fn parse_fields(input: &DeriveInput) -> Result<Vec<ParsedField>> {
    let fields = extract_named_fields(input)?;
    Ok(fields.iter().map(|f| parse_single_field(f)).collect())
}

fn extract_named_fields(input: &DeriveInput) -> Result<Vec<&Field>> {
    match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => Ok(fields.named.iter().collect()),
            _ => Err(syn::Error::new_spanned(
                input,
                "FieldKinds only supports structs with named fields",
            )),
        },
        _ => Err(syn::Error::new_spanned(
            input,
            "FieldKinds only supports structs",
        )),
    }
}

fn parse_single_field(field: &Field) -> ParsedField {
    ParsedField {
        ident: field.ident.clone().unwrap(),
        ty: field.ty.clone(),
        rename: parse_field_rename(field),
        tags: parse_field_tags(field),
        skip: parse_field_skip(field),
    }
}

fn parse_field_rename(field: &Field) -> Option<String> {
    for attr in &field.attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        let mut result = None;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                let value: LitStr = meta.value()?.parse()?;
                result = Some(value.value());
            }
            Ok(())
        });

        if result.is_some() {
            return result;
        }
    }
    None
}

fn parse_field_tags(field: &Field) -> Vec<String> {
    let mut tags = Vec::new();
    for attr in &field.attrs {
        if attr.path().is_ident("field_tags")
            && let Ok(args) = attr.parse_args_with(
                syn::punctuated::Punctuated::<Lit, syn::Token![,]>::parse_terminated,
            )
        {
            tags.extend(args.iter().filter_map(|lit| {
                if let Lit::Str(s) = lit {
                    Some(s.value())
                } else {
                    None
                }
            }));
        }
    }
    tags
}

fn parse_field_skip(field: &Field) -> bool {
    for attr in &field.attrs {
        if attr.path().is_ident("field_kinds")
            && let Ok(meta) = attr.parse_args::<Ident>()
            && meta == "skip"
        {
            return true;
        }
    }
    false
}

fn string_to_rename_rule(s: &str) -> Option<RenameRule<'static>> {
    match s {
        "camelCase" => Some(RenameRule::Case(Case::Camel)),
        "snake_case" => Some(RenameRule::Case(Case::Snake)),
        "PascalCase" => Some(RenameRule::Case(Case::Pascal)),
        "SCREAMING_SNAKE_CASE" => Some(RenameRule::Case(Case::UpperSnake)),
        "kebab-case" => Some(RenameRule::Case(Case::Kebab)),
        "SCREAMING-KEBAB-CASE" => Some(RenameRule::Case(Case::UpperKebab)),
        "lowercase" => Some(RenameRule::Lowercase),
        "UPPERCASE" => Some(RenameRule::Uppercase),
        _ => None,
    }
}
