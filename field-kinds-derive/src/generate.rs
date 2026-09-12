use proc_macro2::{TokenStream, TokenTree};
use quote::{ToTokens, format_ident, quote};
use std::collections::HashSet;
use syn::{
    GenericParam, Generics, Ident, Lifetime, Type, TypeParamBound, Visibility,
};

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

    let marker_names = marker_type_names(&active_fields);
    let markers = generate_markers(&active_fields, &marker_names, generics);
    let field_info_impls = generate_field_info_impls(
        &mod_name,
        &marker_names,
        generics,
        &active_fields,
        rename_all,
        crate_path,
    );
    let visit_impl = generate_visit_impl(
        struct_name,
        generics,
        &active_fields,
        rename_all,
        crate_path,
    );

    // The module only holds self-contained marker definitions. Everything
    // that mentions field types or bounds stays next to the struct, where
    // names resolve exactly as in the struct definition: marker names cannot
    // shadow them, and types declared inside a function remain visible.
    quote! {
        #[doc(hidden)]
        pub mod #mod_name {
            #markers
        }

        #field_info_impls

        #visit_impl
    }
}

fn module_name(struct_name: &Ident) -> Ident {
    use convert_case::{Case, Casing};
    format_ident!("{}_fields", struct_name.to_string().to_case(Case::Snake))
}

/// Marker type names for `fields`, kept unique: different field names can
/// convert to the same `PascalCase` form, as `field1` and `field_1` do.
fn marker_type_names(fields: &[&ParsedField]) -> Vec<Ident> {
    let mut names: Vec<Ident> = Vec::with_capacity(fields.len());

    for field in fields {
        let base = field.marker_type_name();
        let mut name = base.clone();
        let mut suffix: usize = 1;
        while names.contains(&name) {
            suffix += 1;
            name = format_ident!("{}_{}", base, suffix);
        }
        names.push(name);
    }

    names
}

/// Generic parameters of a marker type: the struct's own parameters without
/// bounds or defaults, which only the `FieldInfo` impl needs. Type parameters
/// are `?Sized` so that unsized arguments stay allowed.
fn marker_params(generics: &Generics) -> TokenStream {
    let params = generics.params.iter().map(|p| match p {
        GenericParam::Lifetime(lt) => {
            let lt = &lt.lifetime;
            quote! { #lt }
        }
        GenericParam::Type(tp) => {
            let ident = &tp.ident;
            quote! { #ident: ?::core::marker::Sized }
        }
        GenericParam::Const(cp) => {
            let ident = &cp.ident;
            let ty = &cp.ty;
            quote! { const #ident: #ty }
        }
    });
    quote! { <#(#params),*> }
}

/// Generates a `PhantomData` type over all lifetime and type parameters.
/// Lifetimes become `&'a ()`; each type gets its own `PhantomData`, which
/// accepts unsized types in any position.
fn phantom_data_type(generics: &Generics) -> TokenStream {
    let params = generics.params.iter().filter_map(|p| match p {
        GenericParam::Lifetime(lt) => {
            let lt = &lt.lifetime;
            Some(quote! { &#lt () })
        }
        GenericParam::Type(tp) => {
            let ident = &tp.ident;
            Some(quote! { ::core::marker::PhantomData<#ident> })
        }
        GenericParam::Const(_) => None,
    });
    quote! { ::core::marker::PhantomData<(#(#params,)*)> }
}

/// Visibility of a marker type: the visibility of its field, expressed from
/// inside the generated module. Markers of non-public fields must not be
/// `pub`, or `FieldInfo::Value` would expose a private field type (E0446).
fn marker_visibility(vis: &Visibility) -> TokenStream {
    match vis {
        Visibility::Public(_) => quote! { pub },
        Visibility::Inherited => quote! { pub(super) },
        Visibility::Restricted(restricted) => {
            let path = &restricted.path;
            let path = match path.segments.first() {
                Some(first) if first.ident == "self" => quote! { super },
                Some(first) if first.ident == "super" => {
                    quote! { super::#path }
                }
                _ => quote! { #path },
            };
            quote! { pub(in #path) }
        }
    }
}

fn tags_tokens(tags: &[String]) -> TokenStream {
    if tags.is_empty() {
        quote! { &[] }
    } else {
        quote! { &[#(#tags),*] }
    }
}

/// The field's category marker type: the `#[field_kinds(category = ...)]`
/// override when given, otherwise the one `Categorized` maps the type to.
fn category_type(field: &ParsedField, crate_path: &TokenStream) -> TokenStream {
    field.category.as_ref().map_or_else(
        || {
            let field_type = &field.ty;
            quote! { <#field_type as #crate_path::Categorized>::Category }
        },
        |category| quote! { #category },
    )
}

fn generate_markers(
    fields: &[&ParsedField],
    marker_names: &[Ident],
    generics: &Generics,
) -> TokenStream {
    let params = marker_params(generics);
    let phantom_type = phantom_data_type(generics);

    fields
        .iter()
        .zip(marker_names)
        .map(|(field, type_name)| {
            let vis = marker_visibility(&field.vis);

            // Every parameter, const ones included, must appear on the
            // marker, or the `FieldInfo` impl could not use it.
            if generics.params.is_empty() {
                quote! {
                    #[derive(Debug, Clone, Copy)]
                    #vis struct #type_name;
                }
            } else {
                quote! {
                    #vis struct #type_name #params (#phantom_type);
                }
            }
        })
        .collect()
}

/// Collects the lifetime names and the identifiers `tokens` mention.
fn collect_names(
    tokens: TokenStream,
    lifetimes: &mut HashSet<String>,
    idents: &mut HashSet<String>,
) {
    let mut tokens = tokens.into_iter().peekable();
    while let Some(token) = tokens.next() {
        match token {
            TokenTree::Group(group) => {
                collect_names(group.stream(), lifetimes, idents);
            }
            TokenTree::Punct(punct) if punct.as_char() == '\'' => {
                if let Some(TokenTree::Ident(ident)) = tokens.peek() {
                    lifetimes.insert(ident.to_string());
                    tokens.next();
                }
            }
            TokenTree::Ident(ident) => {
                idents.insert(ident.to_string());
            }
            TokenTree::Punct(_) | TokenTree::Literal(_) => {}
        }
    }
}

/// The lifetime names and the identifiers a type mentions.
fn mentioned_names(ty: &Type) -> (HashSet<String>, HashSet<String>) {
    let mut lifetimes = HashSet::new();
    let mut idents = HashSet::new();
    collect_names(ty.to_token_stream(), &mut lifetimes, &mut idents);
    (lifetimes, idents)
}

/// Impl generics for a `FieldInfo` impl: the struct's parameters, without
/// defaults, carrying the outlives bounds the marker type cannot infer.
///
/// A field like `&'a [T]` gives the struct an inferred `T: 'a` bound, but a
/// marker type holds no such field and infers nothing. Which parameter has to
/// outlive which lifetime cannot be told from the field type alone, so every
/// type parameter it mentions is bound by every lifetime it mentions. The
/// bounds go on the parameters themselves: a `where` clause would name a
/// parameter that its own bounds already name.
fn impl_generics_for(generics: &Generics, field_type: &Type) -> TokenStream {
    if generics.params.is_empty() {
        return quote! {};
    }

    let (lifetime_names, idents) = mentioned_names(field_type);
    let mentioned: Vec<&Lifetime> = generics
        .lifetimes()
        .filter(|param| {
            lifetime_names.contains(&param.lifetime.ident.to_string())
        })
        .map(|param| &param.lifetime)
        .collect();

    let params = generics.params.iter().map(|param| match param {
        GenericParam::Lifetime(lifetime) => quote! { #lifetime },
        GenericParam::Const(const_param) => {
            let mut const_param = const_param.clone();
            const_param.default = None;
            quote! { #const_param }
        }
        GenericParam::Type(type_param) => {
            let mut type_param = type_param.clone();
            type_param.default = None;
            if idents.contains(&type_param.ident.to_string()) {
                for lifetime in &mentioned {
                    type_param
                        .bounds
                        .push(TypeParamBound::Lifetime((*lifetime).clone()));
                }
            }
            quote! { #type_param }
        }
    });

    quote! { <#(#params),*> }
}

fn generate_field_info_impls(
    mod_name: &Ident,
    marker_names: &[Ident],
    generics: &Generics,
    fields: &[&ParsedField],
    rename_all: Option<RenameRule>,
    crate_path: &TokenStream,
) -> TokenStream {
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    fields
        .iter()
        .zip(marker_names)
        .map(|(field, type_name)| {
            let field_type = &field.ty;
            let impl_generics = impl_generics_for(generics, field_type);
            let category = category_type(field, crate_path);
            let field_name_str = field.ident.to_string();
            let serialized_name = field.serialized_name(rename_all);
            let tags_tokens = tags_tokens(&field.tags);

            quote! {
                impl #impl_generics #crate_path::FieldInfo for #mod_name::#type_name #ty_generics #where_clause {
                    const NAME: &'static str = #field_name_str;
                    const SERIALIZED_NAME: &'static str = #serialized_name;
                    const CATEGORY_NAME: &'static str = <#category as #crate_path::TypeCategory>::NAME;
                    const TAGS: &'static [&'static str] = #tags_tokens;

                    type Value = #field_type;
                    type Category = #category;
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
            let category = category_type(f, crate_path);
            let tags_tokens = tags_tokens(&f.tags);
            let skipped = f
                .skip_serializing
                .then(|| quote! { .skipping_serialization() });

            quote! {
                #crate_path::FieldMeta::new(
                    #name,
                    #serialized_name,
                    <#category as #crate_path::TypeCategory>::CATEGORY,
                    #tags_tokens,
                )#skipped
            }
        })
        .collect();

    let names_len = fields.len();
    let names = fields.iter().map(|f| f.ident.to_string());

    // Fields serde leaves out have no serialized name to report.
    let serialized_len = fields.iter().filter(|f| !f.skip_serializing).count();
    let serialized_names = fields
        .iter()
        .filter(|f| !f.skip_serializing)
        .map(|f| f.serialized_name(rename_all));

    quote! {
        impl #impl_generics #crate_path::VisitFields for #struct_name #ty_generics #where_clause {
            const FIELDS: &'static [#crate_path::FieldMeta] = &[
                #(#field_metas),*
            ];

            // Backed by a `static` rather than written as a bare `&[..]`:
            // a `const` is inlined at every use site, and each use would
            // promote its own copy of the array into the binary.
            const NAMES: &'static [&'static str] = {
                static STORAGE: [&'static str; #names_len] = [#(#names),*];
                &STORAGE
            };

            const SERIALIZED_NAMES: &'static [&'static str] = {
                static STORAGE: [&'static str; #serialized_len] =
                    [#(#serialized_names),*];
                &STORAGE
            };
        }
    }
}
