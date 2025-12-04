use proc_macro2::Ident;
use syn::Type;

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
        use convert_case::{Case, Casing};
        quote::format_ident!("{}", self.ident.to_string().to_case(Case::Pascal))
    }

    /// Serialized name considering rename and `rename_all`
    pub fn serialized_name(
        &self,
        rename_all: Option<convert_case::Case>,
    ) -> String {
        use convert_case::Casing;

        self.rename.clone().unwrap_or_else(|| {
            rename_all.map_or_else(
                || self.ident.to_string(),
                |case| self.ident.to_string().to_case(case),
            )
        })
    }
}
