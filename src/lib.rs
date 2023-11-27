extern crate proc_macro;
mod attribute_utils;
mod discover;
mod file_utils;
mod string_utils;
mod token_utils;

use attribute_utils::update_openapi_macro_attributes;
use proc_macro::TokenStream;

use quote::quote;
use string_utils::{discover_paths, extract_pairs};
use syn::parse_macro_input;
use token_utils::{check_macro_placement, extract_paths, output_macro};

/// Macro to automatically discover all the functions with the #[utoipa] attribute
#[proc_macro_attribute]
pub fn utoipa_auto_discovery(
    attributes: proc_macro::TokenStream, // #[utoipa_auto_discovery(paths = "(MODULE_TREE_PATH => MODULE_SRC_PATH) ;")]
    item: proc_macro::TokenStream,       // #[openapi(paths = "")]
) -> proc_macro::TokenStream {
    // (MODULE_TREE_PATH => MODULE_SRC_PATH) ; (MODULE_TREE_PATH => MODULE_SRC_PATH) ; ...
    let paths: String = extract_paths(attributes);
    // [(MODULE_TREE_PATH, MODULE_SRC_PATH)]
    let pairs: Vec<(String, String)> = extract_pairs(paths);

    // #[openapi(...)]
    let mut openapi_macro = parse_macro_input!(item as syn::ItemStruct);

    // Discover all the functions with the #[utoipa] attribute
    let uto_paths: String = discover_paths(pairs);

    // extract the openapi macro attributes : #[openapi(openapi_macro_attibutes)]
    let openapi_macro_attibutes = &mut openapi_macro.attrs;

    // Check if the macro is placed before the #[derive] and #[openapi] attributes
    check_macro_placement(openapi_macro_attibutes.clone());

    // Update the openapi macro attributes with the newly discovered paths
    update_openapi_macro_attributes(openapi_macro_attibutes, &uto_paths);

    // Output the macro back to the compiler
    output_macro(openapi_macro)
}

/// Ignore the function from the auto discovery
#[proc_macro_attribute]
pub fn utoipa_ignore(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as syn::Item);
    let code = quote!(
          #input
    );

    TokenStream::from(code)
}
