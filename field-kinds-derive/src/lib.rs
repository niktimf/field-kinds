use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Meta, Type};

/// Derive macro for compile-time field type introspection.
#[proc_macro_derive(FieldKinds, attributes(field_tags, field_kinds))]
pub fn derive_field_kinds(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // Parse container-level rename_all (from #[serde(...)] or #[field_kinds(...)])
    let rename_all = extract_rename_all(&input.attrs);

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(_) => {
                return syn::Error::new_spanned(
                    &input,
                    "FieldKinds can only be derived for structs with named fields",
                )
                    .to_compile_error()
                    .into();
            }
            Fields::Unit => {
                return syn::Error::new_spanned(
                    &input,
                    "FieldKinds cannot be derived for unit structs",
                )
                    .to_compile_error()
                    .into();
            }
        },
        Data::Enum(_) => {
            return syn::Error::new_spanned(
                &input,
                "FieldKinds can only be derived for structs, not enums",
            )
                .to_compile_error()
                .into();
        }
        Data::Union(_) => {
            return syn::Error::new_spanned(
                &input,
                "FieldKinds can only be derived for structs, not unions",
            )
                .to_compile_error()
                .into();
        }
    };

    let field_entries: Vec<_> = fields
        .iter()
        .filter_map(|f| {
            let field_ident = f.ident.as_ref()?;
            let original_name = field_ident.to_string();

            // Check for #[field_kinds(skip)]
            let should_skip = f.attrs.iter().any(|attr| {
                if attr.path().is_ident("field_kinds") {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("skip") {
                            Ok(())
                        } else {
                            Err(meta.error("expected `skip`"))
                        }
                    })
                        .is_ok()
                } else {
                    false
                }
            });

            if should_skip {
                return None;
            }

            // Determine serialized name:
            // 1. Check #[serde(rename = "...")] or #[field_kinds(rename = "...")]
            // 2. Apply rename_all if no explicit rename
            let serialized_name = extract_field_rename(&f.attrs)
                .unwrap_or_else(|| apply_rename_all(&original_name, rename_all.as_deref()));

            let (type_name, type_path) = extract_type_info(&f.ty);
            let tags = extract_tags(&f.attrs);

            Some(quote! {
                ::field_kinds::FieldMeta {
                    name: #original_name,
                    serialized_name: #serialized_name,
                    type_name: #type_name,
                    type_path: #type_path,
                    tags: &[#(#tags),*],
                }
            })
        })
        .collect();

    let expanded = quote! {
        impl ::field_kinds::FieldKindsInfo for #name {
            const FIELDS: &'static [::field_kinds::FieldMeta] = &[
                #(#field_entries),*
            ];
        }

        impl #name {
            /// Get field metadata by original Rust field name.
            pub fn field(name: &str) -> Option<&'static ::field_kinds::FieldMeta> {
                <Self as ::field_kinds::FieldKindsInfo>::field(name)
            }

            /// Get field metadata by serialized name.
            pub fn field_by_serialized(name: &str) -> Option<&'static ::field_kinds::FieldMeta> {
                <Self as ::field_kinds::FieldKindsInfo>::field_by_serialized(name)
            }

            /// Get all fields metadata.
            pub fn fields() -> &'static [::field_kinds::FieldMeta] {
                <Self as ::field_kinds::FieldKindsInfo>::fields()
            }
        }
    };

    TokenStream::from(expanded)
}

/// Extract rename_all from container attributes (#[serde(rename_all = "...")] or #[field_kinds(rename_all = "...")])
fn extract_rename_all(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        // Check both serde and field_kinds attributes
        if attr.path().is_ident("serde") || attr.path().is_ident("field_kinds") {
            if let Meta::List(meta_list) = &attr.meta {
                let tokens = meta_list.tokens.to_string();
                if let Some(value) = parse_rename_all_value(&tokens) {
                    return Some(value);
                }
            }
        }
    }
    None
}

/// Parse rename_all = "..." from tokens string
fn parse_rename_all_value(tokens: &str) -> Option<String> {
    // Look for rename_all = "..."
    let tokens = tokens.replace(' ', "");
    if let Some(start) = tokens.find("rename_all=") {
        let rest = &tokens[start + 11..]; // len("rename_all=") = 11
        if let Some(value) = extract_quoted_string(rest) {
            return Some(value);
        }
    }
    None
}

/// Extract field-level rename from #[serde(rename = "...")] or #[field_kinds(rename = "...")]
fn extract_field_rename(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("serde") || attr.path().is_ident("field_kinds") {
            if let Meta::List(meta_list) = &attr.meta {
                let tokens = meta_list.tokens.to_string();
                let tokens_clean = tokens.replace(' ', "");

                // Look for rename = "..." but not rename_all
                if let Some(start) = tokens_clean.find("rename=") {
                    // Make sure it's not rename_all
                    let before = if start > 0 { &tokens_clean[start-1..start] } else { "" };
                    if before != "_" {
                        let rest = &tokens_clean[start + 7..]; // len("rename=") = 7
                        if let Some(value) = extract_quoted_string(rest) {
                            return Some(value);
                        }
                    }
                }
            }
        }
    }
    None
}

/// Extract a quoted string value from the start of a string
fn extract_quoted_string(s: &str) -> Option<String> {
    let s = s.trim_start_matches('"');
    if let Some(end) = s.find('"') {
        return Some(s[..end].to_string());
    }
    None
}

/// Apply rename_all transformation to a field name
fn apply_rename_all(name: &str, rename_all: Option<&str>) -> String {
    match rename_all {
        Some("lowercase") => name.to_lowercase(),
        Some("UPPERCASE") => name.to_uppercase(),
        Some("camelCase") => to_camel_case(name, false),
        Some("PascalCase") => to_camel_case(name, true),
        Some("snake_case") => name.to_string(), // already snake_case
        Some("SCREAMING_SNAKE_CASE") => name.to_uppercase(),
        Some("kebab-case") => name.replace('_', "-"),
        Some("SCREAMING-KEBAB-CASE") => name.to_uppercase().replace('_', "-"),
        _ => name.to_string(),
    }
}

/// Convert snake_case to camelCase or PascalCase
fn to_camel_case(name: &str, pascal: bool) -> String {
    let mut result = String::new();
    let mut capitalize_next = pascal;

    for c in name.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

fn extract_type_info(ty: &Type) -> (String, String) {
    match ty {
        Type::Path(type_path) => {
            let type_name = type_path
                .path
                .segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap_or_default();

            let type_path_str = type_path
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");

            (type_name, type_path_str)
        }
        Type::Reference(type_ref) => {
            let (inner_name, inner_path) = extract_type_info(&type_ref.elem);
            (inner_name, format!("&{}", inner_path))
        }
        _ => (String::new(), String::new()),
    }
}

fn extract_tags(attrs: &[syn::Attribute]) -> Vec<String> {
    let mut tags = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("field_tags") {
            if let Meta::List(meta_list) = &attr.meta {
                let tokens = meta_list.tokens.to_string();
                for tag in tokens.split(',') {
                    let tag = tag.trim().trim_matches('"');
                    if !tag.is_empty() {
                        tags.push(tag.to_string());
                    }
                }
            }
        }
    }

    tags
}