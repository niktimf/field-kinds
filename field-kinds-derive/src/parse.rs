use crate::field::{ParsedField, RenameRule};
use proc_macro2::TokenTree;
use syn::ext::IdentExt;
use syn::meta::ParseNestedMeta;
use syn::punctuated::Punctuated;
use syn::{Attribute, DeriveInput, Field, Lit, LitStr, Path, Result, Token};

/// Parses `rename_all` from `#[serde(rename_all = "...")]`
///
/// A rule this macro does not know is reported rather than dropped: the
/// fields would silently keep their original names, and the mismatch with
/// serde would only show up in the serialized output.
pub fn parse_rename_all(attrs: &[Attribute]) -> Result<Option<RenameRule>> {
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        let mut result = None;
        let mut rejected = None;

        // Errors from the serde options this macro does not interpret stay
        // ignored; only an unusable `rename_all` value is reported.
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename_all") {
                if let Some(value) = parse_serialize_value(&meta)? {
                    match string_to_rename_rule(&value.value()) {
                        Some(rule) => result = Some(rule),
                        None => rejected = Some(unknown_rename_rule(&value)),
                    }
                }
            } else {
                skip_meta_value(&meta)?;
            }
            Ok(())
        });

        if let Some(error) = rejected {
            return Err(error);
        }
        if result.is_some() {
            return Ok(result);
        }
    }
    Ok(None)
}

fn unknown_rename_rule(value: &LitStr) -> syn::Error {
    syn::Error::new_spanned(
        value,
        format!(
            "unknown rename_all rule `{}`; expected one of lowercase, \
             UPPERCASE, PascalCase, camelCase, snake_case, \
             SCREAMING_SNAKE_CASE, kebab-case, SCREAMING-KEBAB-CASE",
            value.value()
        ),
    )
}

/// Parses all fields of the struct
pub fn parse_fields(input: &DeriveInput) -> Result<Vec<ParsedField>> {
    let fields = extract_named_fields(input)?;
    fields.iter().map(|f| parse_single_field(f)).collect()
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

fn parse_single_field(field: &Field) -> Result<ParsedField> {
    let options = parse_field_kinds_options(field)?;
    Ok(ParsedField {
        ident: field.ident.as_ref().unwrap().unraw(),
        vis: field.vis.clone(),
        ty: field.ty.clone(),
        rename: parse_field_rename(field),
        tags: parse_field_tags(field)?,
        category: options.category,
        skip_serializing: parse_serde_skip(field),
        skip: options.skip,
    })
}

fn parse_field_rename(field: &Field) -> Option<String> {
    for attr in &field.attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        let mut result = None;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                result =
                    parse_serialize_value(&meta)?.map(|value| value.value());
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
fn parse_serialize_value(meta: &ParseNestedMeta) -> Result<Option<LitStr>> {
    if meta.input.peek(Token![=]) {
        return Ok(Some(meta.value()?.parse()?));
    }

    let mut serialize = None;
    meta.parse_nested_meta(|nested| {
        if nested.path.is_ident("serialize") {
            serialize = Some(nested.value()?.parse()?);
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

/// Parses `#[field_tags("...", "...")]`.
///
/// Like `#[field_kinds(..)]`, this attribute belongs to this crate, so
/// anything other than a list of string literals is a mistake rather than
/// syntax owned by someone else. Both rejections used to be silent, and the
/// costlier one was a missing pair of quotes: `#[field_tags(primary)]` does
/// not parse as a literal at all, so the whole attribute was dropped and the
/// field lost every tag it was given.
fn parse_field_tags(field: &Field) -> Result<Vec<String>> {
    let mut tags = Vec::new();
    for attr in &field.attrs {
        if !attr.path().is_ident("field_tags") {
            continue;
        }

        let args = attr
            .parse_args_with(Punctuated::<Lit, Token![,]>::parse_terminated)
            .map_err(|error| {
                syn::Error::new(
                    error.span(),
                    format!(
                        "{error}; field_tags expects string literals, \
                             as in #[field_tags(\"primary\")]"
                    ),
                )
            })?;

        for arg in &args {
            match arg {
                Lit::Str(tag) => tags.push(tag.value()),
                other => {
                    return Err(syn::Error::new_spanned(
                        other,
                        "field_tags expects string literals, as in \
                         #[field_tags(\"primary\")]",
                    ));
                }
            }
        }
    }
    Ok(tags)
}

/// Options of `#[field_kinds(...)]`.
#[derive(Default)]
struct FieldKindsOptions {
    category: Option<Path>,
    skip: bool,
}

/// Parses `#[field_kinds(...)]`.
///
/// Unlike `#[serde(...)]`, this attribute belongs to this crate, so an
/// option it does not know is a typo rather than syntax owned by someone
/// else, and dropping it silently would make `#[field_kinds(skipp)]` do
/// nothing at all.
fn parse_field_kinds_options(field: &Field) -> Result<FieldKindsOptions> {
    let mut options = FieldKindsOptions::default();
    for attr in &field.attrs {
        if !attr.path().is_ident("field_kinds") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                options.skip = true;
            } else if meta.path.is_ident("category") {
                options.category = Some(meta.value()?.parse()?);
            } else {
                return Err(meta.error(
                    "unknown field_kinds option; expected `skip` or \
                     `category = ...`",
                ));
            }
            Ok(())
        })?;
    }
    Ok(options)
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
