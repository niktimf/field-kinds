use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{GenericParam, Generics, Ident};

use crate::field::{ParsedField, RenameRule};

pub fn generate_all(
    struct_name: &Ident,
    generics: &Generics,
    fields: &[ParsedField],
    rename_all: Option<RenameRule>,
    crate_path: &TokenStream,
) -> TokenStream {
    let mod_name = module_name(struct_name);
    let active_fields: Vec<_> = fields.iter().filter(|f| !f.skip).collect();

    let field_types =
        generate_field_types(&active_fields, rename_all, generics, crate_path);
    let visit_impl = generate_visit_impl(
        struct_name,
        generics,
        &active_fields,
        rename_all,
        crate_path,
    );
    let field_kinds_impl = generate_field_kinds_impl(
        struct_name,
        generics,
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

/// Checks whether generics contain any lifetime or type parameters.
fn has_phantom_params(generics: &Generics) -> bool {
    generics
        .params
        .iter()
        .any(|p| matches!(p, GenericParam::Lifetime(_) | GenericParam::Type(_)))
}

/// Generates a `PhantomData` type wrapping a tuple of all generic parameters.
/// Lifetimes become `&'a ()`, types are used as-is.
fn phantom_data_type(generics: &Generics) -> TokenStream {
    let params: Vec<TokenStream> = generics
        .params
        .iter()
        .filter_map(|p| match p {
            GenericParam::Lifetime(lt) => {
                let lt = &lt.lifetime;
                Some(quote! { &#lt () })
            }
            GenericParam::Type(tp) => {
                let ident = &tp.ident;
                Some(quote! { #ident })
            }
            GenericParam::Const(_) => None,
        })
        .collect();

    if params.len() == 1 {
        let single = &params[0];
        quote! { core::marker::PhantomData<#single> }
    } else {
        quote! { core::marker::PhantomData<(#(#params),*)> }
    }
}

fn generate_field_types(
    fields: &[&ParsedField],
    rename_all: Option<RenameRule>,
    generics: &Generics,
    _crate_path: &TokenStream,
) -> TokenStream {
    let has_generics = has_phantom_params(generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    fields
        .iter()
        .map(|field| {
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

            if has_generics {
                let phantom_type = phantom_data_type(generics);

                quote! {
                    pub struct #type_name #impl_generics (#phantom_type) #where_clause;

                    impl #impl_generics FieldInfo for #type_name #ty_generics #where_clause {
                        const NAME: &'static str = #field_name_str;
                        const SERIALIZED_NAME: &'static str = #serialized_name;
                        const CATEGORY_NAME: &'static str = <<#field_type as Categorized>::Category as TypeCategory>::NAME;
                        const TAGS: &'static [&'static str] = #tags_tokens;

                        type Value = #field_type;
                        type Category = <#field_type as Categorized>::Category;
                    }
                }
            } else {
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
            }
        })
        .collect()
}

fn generate_visit_impl(
    struct_name: &Ident,
    generics: &Generics,
    fields: &[&ParsedField],
    rename_all: Option<RenameRule>,
    crate_path: &TokenStream,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

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
        impl #impl_generics #crate_path::VisitFields for #struct_name #ty_generics #where_clause {
            const FIELDS: &'static [#crate_path::FieldMeta] = &[
                #(#field_metas),*
            ];
        }
    }
}

fn generate_field_kinds_impl(
    struct_name: &Ident,
    generics: &Generics,
    mod_name: &Ident,
    fields: &[&ParsedField],
    crate_path: &TokenStream,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let field_count = fields.len();

    let has_generics = has_phantom_params(generics);
    let hlist_type = fields.iter().rev().fold(quote! { #crate_path::HNil }, |acc, field| {
        let type_name = field.marker_type_name();
        if has_generics {
            quote! { #crate_path::HCons<#mod_name::#type_name #ty_generics, #acc> }
        } else {
            quote! { #crate_path::HCons<#mod_name::#type_name, #acc> }
        }
    });

    quote! {
        impl #impl_generics #crate_path::FieldKinds for #struct_name #ty_generics #where_clause {
            type Fields = #hlist_type;
            const FIELD_COUNT: usize = #field_count;
        }
    }
}
