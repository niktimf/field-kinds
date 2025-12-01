use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, Field, Fields, Ident, Lit, Meta, parse_macro_input};

#[proc_macro_derive(FieldKinds, attributes(field_kinds, field_tags, serde))]
pub fn derive_field_kinds(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match generate(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate(input: DeriveInput) -> syn::Result<TokenStream2> {
    let struct_name = &input.ident;
    let fields = extract_fields(&input)?;

    // Парсим rename_all из #[serde(rename_all = "...")]
    let rename_all = parse_rename_all(&input.attrs);

    // Фильтруем skip поля
    let fields: Vec<_> = fields.into_iter().filter(|f| !is_skipped(f)).collect();

    // Генерируем имя модуля: Order -> order_fields
    let mod_name = format_ident!("{}_fields", struct_name.to_string().to_case(Case::Snake));

    // Определяем путь к крейту через proc-macro-crate
    let crate_path = match crate_name("field-kinds") {
        Ok(FoundCrate::Itself) => quote! { crate },
        Ok(FoundCrate::Name(name)) => {
            let ident = format_ident!("{}", name);
            quote! { ::#ident }
        }
        Err(_) => quote! { ::field_kinds },
    };

    // Генерируем типы полей
    let field_types = generate_field_types(&fields, &rename_all)?;

    // Генерируем HList тип
    let hlist_type = generate_hlist_type(&fields);

    // Генерируем impl VisitFields
    let visit_impl = generate_visit_impl(struct_name, &mod_name, &fields, &crate_path);

    // Генерируем impl FieldKinds
    let field_kinds_impl = generate_field_kinds_impl(struct_name, &mod_name, &fields, &crate_path);

    Ok(quote! {
        pub mod #mod_name {
            use super::*;
            use #crate_path::{FieldInfo, Categorized, TypeCategory};

            #field_types
        }

        #hlist_type

        #visit_impl

        #field_kinds_impl
    })
}

fn extract_fields(input: &DeriveInput) -> syn::Result<Vec<&Field>> {
    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => Ok(fields.named.iter().collect()),
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

fn parse_rename_all(attrs: &[Attribute]) -> Option<Case> {
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
                        return match s.value().as_str() {
                            "camelCase" => Some(Case::Camel),
                            "snake_case" => Some(Case::Snake),
                            "PascalCase" => Some(Case::Pascal),
                            "SCREAMING_SNAKE_CASE" => Some(Case::UpperSnake),
                            "kebab-case" => Some(Case::Kebab),
                            _ => None,
                        };
                    }
                }
            }
        }
    }
    None
}

fn is_skipped(field: &Field) -> bool {
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

fn generate_field_types(fields: &[&Field], rename_all: &Option<Case>) -> syn::Result<TokenStream2> {
    let mut tokens = TokenStream2::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        // Имя типа: customer_name -> CustomerName
        let type_name = format_ident!("{}", field_name.to_string().to_case(Case::Pascal));

        // Serialized имя
        let serialized_name = if let Some(renamed) = parse_field_rename(field) {
            renamed
        } else if let Some(case) = rename_all {
            field_name.to_string().to_case(*case)
        } else {
            field_name.to_string()
        };

        // Теги
        let tags = parse_field_tags(field);
        let tags_tokens = if tags.is_empty() {
            quote! { &[] }
        } else {
            quote! { &[#(#tags),*] }
        };

        let field_name_str = field_name.to_string();

        tokens.extend(quote! {
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
        });
    }

    Ok(tokens)
}

fn generate_hlist_type(fields: &[&Field]) -> TokenStream2 {
    if fields.is_empty() {
        return quote! {};
    }

    // Строим HCons<A, HCons<B, HCons<C, HNil>>> справа налево
    let mut hlist = quote! { ::frunk::HNil };

    for field in fields.iter().rev() {
        let field_name = field.ident.as_ref().unwrap();
        let type_name = format_ident!("{}", field_name.to_string().to_case(Case::Pascal));
        let mod_prefix = quote! {}; // тип уже в scope через mod

        hlist = quote! {
            ::frunk::HCons<#mod_prefix #type_name, #hlist>
        };
    }

    quote! {} // HList тип используется только в impl, не нужен отдельно
}

fn generate_visit_impl(
    struct_name: &Ident,
    mod_name: &Ident,
    fields: &[&Field],
    crate_path: &TokenStream2,
) -> TokenStream2 {
    let visit_calls: Vec<_> = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.as_ref().unwrap();
            let type_name = format_ident!("{}", field_name.to_string().to_case(Case::Pascal));
            quote! {
                visitor.visit::<#mod_name::#type_name>();
            }
        })
        .collect();

    quote! {
        impl #crate_path::VisitFields for #struct_name {
            fn visit_fields<V: #crate_path::FieldVisitor>(visitor: &mut V) {
                #(#visit_calls)*
            }
        }
    }
}

fn generate_field_kinds_impl(
    struct_name: &Ident,
    mod_name: &Ident,
    fields: &[&Field],
    crate_path: &TokenStream2,
) -> TokenStream2 {
    use convert_case::{Case, Casing};

    let field_count = fields.len();

    // Генерируем HList тип для Fields
    let hlist_type = if fields.is_empty() {
        quote! { ::frunk::HNil }
    } else {
        let mut hlist = quote! { ::frunk::HNil };
        for field in fields.iter().rev() {
            let field_name = field.ident.as_ref().unwrap();
            let type_name = format_ident!("{}", field_name.to_string().to_case(Case::Pascal));
            hlist = quote! {
                ::frunk::HCons<#mod_name::#type_name, #hlist>
            };
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
