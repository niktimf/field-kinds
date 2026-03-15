use convert_case::Case;
use proc_macro2::Ident;
use syn::Type;

/// Rename rule matching serde's `rename_all` variants.
#[derive(Clone, Copy)]
pub enum RenameRule<'a> {
    /// Delegates to `convert_case::Case` for case conversions
    /// that split on word boundaries.
    Case(Case<'a>),
    /// serde's `lowercase`: simple `str::to_lowercase()`.
    Lowercase,
    /// serde's `UPPERCASE`: simple `str::to_uppercase()`.
    Uppercase,
}

pub struct ParsedField {
    pub ident: Ident,
    pub ty: Type,
    pub rename: Option<String>,
    pub tags: Vec<String>,
    pub skip: bool,
}

impl ParsedField {
    /// Name of the marker type: `user_name` -> `UserName`
    pub fn marker_type_name(&self) -> Ident {
        use convert_case::Casing;
        quote::format_ident!("{}", self.ident.to_string().to_case(Case::Pascal))
    }

    /// Serialized name considering rename and `rename_all`
    pub fn serialized_name(&self, rename_all: Option<RenameRule>) -> String {
        use convert_case::Casing;

        self.rename.clone().unwrap_or_else(|| {
            let name = self.ident.to_string();
            match rename_all {
                None => name,
                Some(RenameRule::Case(case)) => name.to_case(case),
                Some(RenameRule::Lowercase) => name.to_lowercase(),
                Some(RenameRule::Uppercase) => name.to_uppercase(),
            }
        })
    }
}
