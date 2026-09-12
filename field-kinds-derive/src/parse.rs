use crate::field::{ParsedField, RenameRule};
use proc_macro2::TokenTree;
use syn::ext::IdentExt;
use syn::meta::ParseNestedMeta;
use syn::{Attribute, DeriveInput, Field, Lit, LitStr, Path, Result, Token};

/// Parses `rename_all` from `#[serde(rename_all = "...")]`
pub fn parse_rename_all(attrs: &[Attribute]) -> Option<RenameRule> {
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        let mut result = None;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename_all") {
                result = parse_serialize_value(&meta)?
                    .and_then(|value| string_to_rename_rule(&value));
            } else {
                skip_meta_value(&meta)?;
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
    let options = parse_field_kinds_options(field);
    ParsedField {
        ident: field.ident.as_ref().unwrap().unraw(),
        vis: field.vis.clone(),
        ty: field.ty.clone(),
        rename: parse_field_rename(field),
        tags: parse_field_tags(field),
        category: options.category,
        skip_serializing: parse_serde_skip(field),
        skip: options.skip,
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
                result = parse_serialize_value(&meta)?;
            } else {
                skip_meta_value(&meta)?;
            }
            Ok(())
        });

        if result.is_some() {
            return result;
        }
    }
    None
}

/// Whether serde leaves the field out of the serialized output, i.e. it
/// carries `#[serde(skip)]` or `#[serde(skip_serializing)]`.
///
/// `skip_serializing_if` is decided at run time and does not count.
fn parse_serde_skip(field: &Field) -> bool {
    let mut skipped = false;
    for attr in &field.attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip")
                || meta.path.is_ident("skip_serializing")
            {
                skipped = true;
            } else {
                skip_meta_value(&meta)?;
            }
            Ok(())
        });
    }
    skipped
}

/// Reads the serialization-side value of a serde option written either as
/// `key = "..."` or as `key(serialize = "...", deserialize = "...")`.
fn parse_serialize_value(meta: &ParseNestedMeta) -> Result<Option<String>> {
    if meta.input.peek(Token![=]) {
        let value: LitStr = meta.value()?.parse()?;
        return Ok(Some(value.value()));
    }

    let mut serialize = None;
    meta.parse_nested_meta(|nested| {
        if nested.path.is_ident("serialize") {
            let value: LitStr = nested.value()?.parse()?;
            serialize = Some(value.value());
        } else {
            skip_meta_value(&nested)?;
        }
        Ok(())
    })?;
    Ok(serialize)
}

/// Consumes the value of a serde option this macro does not interpret,
/// e.g. `= "Option::is_none"` or `(serialize = "T: Clone")`.
///
/// `parse_nested_meta` fails on the first value left unconsumed, which
/// would hide every option after it in the same attribute.
fn skip_meta_value(meta: &ParseNestedMeta) -> Result<()> {
    while !meta.input.is_empty() && !meta.input.peek(Token![,]) {
        meta.input.parse::<TokenTree>()?;
    }
    Ok(())
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

/// Options of `#[field_kinds(...)]`.
#[derive(Default)]
struct FieldKindsOptions {
    category: Option<Path>,
    skip: bool,
}

fn parse_field_kinds_options(field: &Field) -> FieldKindsOptions {
    let mut options = FieldKindsOptions::default();
    for attr in &field.attrs {
        if !attr.path().is_ident("field_kinds") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                options.skip = true;
            } else if meta.path.is_ident("category") {
                options.category = Some(meta.value()?.parse()?);
            } else {
                skip_meta_value(&meta)?;
            }
            Ok(())
        });
    }
    options
}

fn string_to_rename_rule(s: &str) -> Option<RenameRule> {
    match s {
        "camelCase" => Some(RenameRule::Camel),
        "snake_case" => Some(RenameRule::Snake),
        "PascalCase" => Some(RenameRule::Pascal),
        "SCREAMING_SNAKE_CASE" => Some(RenameRule::ScreamingSnake),
        "kebab-case" => Some(RenameRule::Kebab),
        "SCREAMING-KEBAB-CASE" => Some(RenameRule::ScreamingKebab),
        "lowercase" => Some(RenameRule::Lowercase),
        "UPPERCASE" => Some(RenameRule::Uppercase),
        _ => None,
    }
}
