use convert_case::Case;
use syn::{Attribute, DeriveInput, Field, Ident, Lit, Meta, Result};

use crate::field::ParsedField;

/// Parses rename_all from #[serde(rename_all = "...")]
pub fn parse_rename_all(attrs: &[Attribute]) -> Option<Case> {
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        let nested = attr
            .parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated)
            .ok()?;

        for meta in nested {
            if let Meta::NameValue(nv) = meta {
                if nv.path.is_ident("rename_all") {
                    if let syn::Expr::Lit(syn::ExprLit {
                        lit: Lit::Str(s), ..
                    }) = nv.value
                    {
                        let val = s.value();
                        let case: Option<Case> = string_to_case(&val);
                        return case;
                    }
                }
            }
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

        let nested = attr
            .parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated)
            .ok()?;

        for meta in nested {
            if let Meta::NameValue(nv) = meta {
                if nv.path.is_ident("rename") {
                    if let syn::Expr::Lit(syn::ExprLit {
                        lit: Lit::Str(s), ..
                    }) = nv.value
                    {
                        return Some(s.value());
                    }
                }
            }
        }
    }
    None
}

fn parse_field_tags(field: &Field) -> Vec<String> {
    for attr in &field.attrs {
        if attr.path().is_ident("field_tags") {
            if let Ok(args) = attr.parse_args_with(
                syn::punctuated::Punctuated::<Lit, syn::Token![,]>::parse_terminated,
            ) {
                return args
                    .iter()
                    .filter_map(|lit| {
                        if let Lit::Str(s) = lit {
                            Some(s.value())
                        } else {
                            None
                        }
                    })
                    .collect();
            }
        }
    }
    Vec::new()
}

fn parse_field_skip(field: &Field) -> bool {
    for attr in &field.attrs {
        if attr.path().is_ident("field_kinds") {
            if let Ok(meta) = attr.parse_args::<Ident>() {
                if meta == "skip" {
                    return true;
                }
            }
        }
    }
    false
}

fn string_to_case(s: &str) -> Option<Case<'static>> {
    match s {
        "camelCase" => Some(Case::Camel),
        "snake_case" => Some(Case::Snake),
        "PascalCase" => Some(Case::Pascal),
        "SCREAMING_SNAKE_CASE" => Some(Case::UpperSnake),
        "kebab-case" => Some(Case::Kebab),
        _ => None,
    }
}
