use convert_case::Case;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::field::ParsedField;

pub fn generate_all(
    struct_name: &Ident,
    fields: &[ParsedField],
    rename_all: Option<Case>,
    crate_path: &TokenStream,
) -> TokenStream {
    let mod_name = module_name(struct_name);
    let active_fields: Vec<_> = fields.iter().filter(|f| !f.skip).collect();

    let field_types =
        generate_field_types(&active_fields, rename_all, crate_path);
    let visit_impl = generate_visit_impl(
        struct_name,
        &active_fields,
        rename_all,
        crate_path,
    );
    let field_kinds_impl = generate_field_kinds_impl(
        struct_name,
        &mod_name,
        &active_fields,
        crate_path,
    );

    quote! {
        pub mod #mod_name {
            use super::*;
            use #crate_path::{FieldInfo, Categorized, TypeCategory};

            #field_types
        }

        #visit_impl
        #field_kinds_impl
    }
}

fn module_name(struct_name: &Ident) -> Ident {
    use convert_case::{Case, Casing};
    format_ident!("{}_fields", struct_name.to_string().to_case(Case::Snake))
}

fn generate_field_types(
    fields: &[&ParsedField],
    rename_all: Option<Case>,
    _crate_path: &TokenStream,
) -> TokenStream {
    fields.iter().map(|field| {
        let type_name = field.marker_type_name();
        let field_type = &field.ty;
        let field_name_str = field.ident.to_string();
        let serialized_name = field.serialized_name(rename_all);

        let tags = &field.tags;
        let tags_tokens = if tags.is_empty() {
            quote! { &[] }
        } else {
            quote! { &[#(#tags),*] }
        };

        quote! {
            #[derive(Debug, Clone, Copy)]
            pub struct #type_name;

            impl FieldInfo for #type_name {
                const NAME: &'static str = #field_name_str;
                const SERIALIZED_NAME: &'static str = #serialized_name;
                const CATEGORY_NAME: &'static str = <<#field_type as Categorized>::Category as TypeCategory>::NAME;
                const TAGS: &'static [&'static str] = #tags_tokens;

                type Value = #field_type;
                type Category = <#field_type as Categorized>::Category;
            }
        }
    }).collect()
}

fn generate_visit_impl(
    struct_name: &Ident,
    fields: &[&ParsedField],
    rename_all: Option<Case>,
    crate_path: &TokenStream,
) -> TokenStream {
    let field_metas: Vec<_> = fields
        .iter()
        .map(|f| {
            let name = f.ident.to_string();
            let serialized_name = f.serialized_name(rename_all);
            let field_type = &f.ty;
            let tags = &f.tags;
            let tags_tokens = if tags.is_empty() {
                quote! { &[] }
            } else {
                quote! { &[#(#tags),*] }
            };

            quote! {
                #crate_path::FieldMeta {
                    name: #name,
                    serialized_name: #serialized_name,
                    category: <<#field_type as #crate_path::Categorized>::Category as #crate_path::TypeCategory>::NAME,
                    tags: #tags_tokens,
                }
            }
        })
        .collect();

    quote! {
        impl #crate_path::VisitFields for #struct_name {
            const FIELDS: &'static [#crate_path::FieldMeta] = &[
                #(#field_metas),*
            ];
        }
    }
}

fn generate_field_kinds_impl(
    struct_name: &Ident,
    mod_name: &Ident,
    fields: &[&ParsedField],
    crate_path: &TokenStream,
) -> TokenStream {
    let field_count = fields.len();

    let hlist_type = if fields.is_empty() {
        quote! { ::frunk::HNil }
    } else {
        let mut hlist = quote! { ::frunk::HNil };
        for field in fields.iter().rev() {
            let type_name = field.marker_type_name();
            hlist = quote! { ::frunk::HCons<#mod_name::#type_name, #hlist> };
        }
        hlist
    };

    quote! {
        impl #crate_path::FieldKinds for #struct_name {
            type Fields = #hlist_type;
            const FIELD_COUNT: usize = #field_count;
        }
    }
}
