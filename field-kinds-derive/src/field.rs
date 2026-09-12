use convert_case::Case;
use proc_macro2::Ident;
use syn::Type;

/// Rename rule matching serde's `rename_all` variants.
#[derive(Clone, Copy)]
pub enum RenameRule {
    Lowercase,
    Uppercase,
    Pascal,
    Camel,
    Snake,
    ScreamingSnake,
    Kebab,
    ScreamingKebab,
}

impl RenameRule {
    /// Renames a struct field the way serde does.
    ///
    /// serde assumes fields are already `snake_case`, so it only rewrites
    /// underscores and ASCII case and never splits words on digits or case
    /// changes: `address_line1` becomes `ADDRESS_LINE1`, not `ADDRESS_LINE_1`.
    fn apply_to_field(self, field: &str) -> String {
        match self {
            Self::Lowercase | Self::Snake => field.to_owned(),
            Self::Uppercase | Self::ScreamingSnake => {
                field.to_ascii_uppercase()
            }
            Self::Pascal => {
                let mut pascal = String::with_capacity(field.len());
                let mut capitalize = true;
                for ch in field.chars() {
                    if ch == '_' {
                        capitalize = true;
                    } else if capitalize {
                        pascal.push(ch.to_ascii_uppercase());
                        capitalize = false;
                    } else {
                        pascal.push(ch);
                    }
                }
                pascal
            }
            Self::Camel => {
                let mut camel = Self::Pascal.apply_to_field(field);
                if let Some(first) = camel.get_mut(..1) {
                    first.make_ascii_lowercase();
                }
                camel
            }
            Self::Kebab => field.replace('_', "-"),
            Self::ScreamingKebab => {
                field.to_ascii_uppercase().replace('_', "-")
            }
        }
    }
}

pub struct ParsedField {
    /// Field name without the `r#` prefix, as serde reports it.
    pub ident: Ident,
    pub vis: syn::Visibility,
    pub ty: Type,
    pub rename: Option<String>,
    pub tags: Vec<String>,
    pub category: Option<syn::Path>,
    pub skip_serializing: bool,
    pub skip: bool,
}

impl ParsedField {
    /// Name of the marker type: `user_name` -> `UserName`
    ///
    /// Falls back to the field's own name when the `PascalCase` form cannot
    /// name a type: `__` converts to nothing and `_1` to `1`. `Self` is a
    /// keyword, so it gains a trailing underscore.
    pub fn marker_type_name(&self) -> Ident {
        use convert_case::Casing;

        let name = self.ident.to_string();
        let pascal = name.to_case(Case::Pascal);

        let marker = if pascal.is_empty()
            || pascal.starts_with(|c: char| c.is_ascii_digit())
        {
            name
        } else if pascal == "Self" {
            format!("{pascal}_")
        } else {
            pascal
        };

        quote::format_ident!("{}", marker)
    }

    /// Serialized name considering rename and `rename_all`
    pub fn serialized_name(&self, rename_all: Option<RenameRule>) -> String {
        self.rename.clone().unwrap_or_else(|| {
            let name = self.ident.to_string();
            match rename_all {
                Some(rule) => rule.apply_to_field(&name),
                None => name,
            }
        })
    }
}
