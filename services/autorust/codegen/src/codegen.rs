use crate::{
    identifier::ident,
    spec::{self, RefKey, TypeName},
    Config, PropertyName, Spec,
};
use heck::ToPascalCase;
use once_cell::sync::Lazy;
use proc_macro2::TokenStream;
use quote::quote;
use regex::Regex;
use std::path::{Path, PathBuf};

/// code generation context
pub struct CodeGen {
    config: Config,
    pub spec: Spec,
}

impl CodeGen {
    pub fn new(config: Config) -> Result<Self, Error> {
        let spec = Spec::read_files(&config.input_files).map_err(Error::Spec)?;
        Ok(Self { config, spec })
    }

    pub fn input_files(&self) -> &[PathBuf] {
        &self.config.input_files
    }

    pub fn output_folder(&self) -> &Path {
        &self.config.output_folder
    }

    pub fn should_workaround_case(&self) -> bool {
        if let Some(title) = self.spec.title() {
            self.config.fix_case_properties.contains(title)
        } else {
            false
        }
    }

    pub fn should_force_optional(&self, prop_nm: &PropertyName) -> bool {
        self.config.optional_properties.contains(prop_nm)
    }

    pub fn should_force_obj(&self, prop_nm: &PropertyName) -> bool {
        self.config.invalid_types.contains(prop_nm)
    }

    pub fn should_box_property(&self, prop_nm: &PropertyName) -> bool {
        self.config.box_properties.contains(prop_nm)
    }

    pub fn get_request_content_type_json(&self) -> String {
        let consumes = self.spec.consumes();
        consumes
            .into_iter()
            .filter(|x| x.starts_with("application/json"))
            .map(|x| x.to_string())
            .next()
            .unwrap_or_else(|| "application/json".to_string())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SpecError: {0}")]
    Spec(#[from] spec::Error),
    #[error("creating function name: {0}")]
    FunctionName(#[source] crate::identifier::Error),
    #[error("creating type name for schema ref: {0}")]
    TypeNameForSchemaRef(#[source] crate::identifier::Error),
    #[error("creating name for status code: {0}")]
    StatusCodeName(#[source] crate::identifier::Error),
    #[error("creating type name for response: {0}")]
    ResponseTypeName(#[source] crate::identifier::Error),
    #[error("creating type for response: {0}")]
    ResponseType(#[source] crate::status_codes::Error),
    #[error("creating name for param: {0}")]
    ParamName(#[source] crate::identifier::Error),
    #[error("creating name for property: {0}")]
    PropertyName(#[source] crate::identifier::Error),
    #[error("creating name for module: {0}")]
    ModuleName(#[source] crate::identifier::Error),
    #[error("creating name for enum variant: {0}")]
    EnumVariantName(#[source] crate::identifier::Error),
    #[error("creating name for enum {property}: {source}")]
    EnumName {
        source: crate::identifier::Error,
        property: String,
    },
    #[error("creating name for enum value {property}: {source}")]
    EnumValueName {
        source: crate::identifier::Error,
        property: String,
    },
    #[error("creating name for Vec alias: {0}")]
    VecAliasName(#[source] crate::identifier::Error),
    #[error("creating name for struct: {0}")]
    StructName(#[source] crate::identifier::Error),
    #[error("creating name for field in struct: {0}")]
    StructFieldName(#[source] crate::identifier::Error),
    #[error("api-version is missing")]
    MissingApiVersion,
    #[error("operation {0} is missing an x-ms-examples")]
    OperationMissingExample(String),
    #[error("operation is missing responses")]
    OperationMissingResponses,
    #[error("creating path for example {0}")]
    ExamplePath(#[source] crate::path::Error),
    #[error("example path not utf8")]
    ExamplePathNotUtf8,
    #[error("status code required")]
    StatusCodeRequired,
    #[error("creating name for examples")]
    ExamplesName(#[source] crate::identifier::Error),
    #[error("status code: {0}")]
    StatusCode(#[from] crate::status_codes::Error),
    #[error("creating type name for schema: {0}")]
    TypeNameForSchema(#[source] crate::spec::Error),
    #[error("array items: {0}")]
    ArrayItems(#[source] crate::spec::Error),
    #[error("no ref key")]
    NoRefKey,
    #[error("RefKey not found {0:?}", ref_key)]
    RefKeyNotFound { ref_key: RefKey },
}

pub fn is_vec(ts: &TokenStream) -> bool {
    ts.to_string().starts_with("Vec <")
}

/// A header placed at the top the file to say that it is generated by AutoRust.
pub fn create_generated_by_header() -> TokenStream {
    let version = env!("CARGO_PKG_VERSION");
    let comment = format!("generated by AutoRust {}", &version);
    quote! { #![doc = #comment] }
}

/// Wraps a type in an Option
pub fn add_option(is_option: bool, tp: TokenStream) -> TokenStream {
    if is_option {
        quote! { Option<#tp> }
    } else {
        tp
    }
}

pub fn type_name_gen(type_name: &TypeName, as_ref: bool, qualify_models: bool) -> Result<TokenStream, Error> {
    Ok(match type_name {
        TypeName::Reference(name) => {
            let idt = ident(&name.to_pascal_case()).map_err(Error::TypeNameForSchemaRef)?;
            let idt = if qualify_models {
                quote! { models::#idt }
            } else {
                idt
            };
            match as_ref {
                true => quote! { impl Into<#idt> },
                false => idt,
            }
        }
        TypeName::Array(vec_items_typ) => {
            let vec_items_typ = type_name_gen(vec_items_typ, false, qualify_models)?;
            match as_ref {
                true => quote! { impl Into<Vec<#vec_items_typ>> },
                false => quote! { Vec<#vec_items_typ> },
            }
        }
        TypeName::Value => match as_ref {
            true => quote! { impl Into<serde_json::Value> },
            false => quote! { serde_json::Value },
        },
        TypeName::Bytes => match as_ref {
            true => quote! { impl Into<bytes::Bytes> },
            false => quote! { bytes::Bytes },
        },
        TypeName::Int32 => quote! { i32 },
        TypeName::Int64 => quote! { i64 },
        TypeName::Float32 => quote! { f32 },
        TypeName::Float64 => quote! { f64 },
        TypeName::Boolean => quote! { bool },
        TypeName::String => match as_ref {
            true => quote! { impl Into<String> },
            false => quote! { String },
        },
    })
}

pub fn create_mod() -> TokenStream {
    quote! {
        pub mod models;
        pub mod operations;
    }
}

// any word character or `-` between curly braces
pub static PARAM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{([\w-]+)\}").unwrap());

pub fn parse_params(path: &str) -> Vec<String> {
    // capture 0 is the whole match and 1 is the actual capture like other languages
    PARAM_RE.captures_iter(path).into_iter().map(|c| c[1].to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_params_keyvault() -> Result<(), Error> {
        assert_eq!(
            parse_params("/storage/{storage-account-name}/sas/{sas-definition-name}"),
            vec!["storage-account-name".to_owned(), "sas-definition-name".to_owned()]
        );
        Ok(())
    }
}
