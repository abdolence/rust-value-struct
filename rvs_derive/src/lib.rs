//! Value Structs derive macros for Rust to support the newtype pattern
//!
//! A very simple derive macros to support strong type system and
//! the new type pattern (https://doc.rust-lang.org/1.0.0/style/features/types/newtype.html).
//!
//! For example:
//! ```
//! use rvs_derive::ValueStruct;
//!
//! #[derive(ValueStruct)]
//! struct UserId(String);
//!
//! let uid : UserId = "my-uid".into();
//! ```
//!
//! `ValueStruct` generates for you:
//!  - `std::convert::From<>` instances automatically to help you to create your structs.
//!  - an inline `value()` function to access your field directly without using .0.
//!
//! There are different behaviour for different field types:
//! - for `std::string::String` it generates additional instance for `From<&str>`
//! - for scalar types `value()` the result type isn't a reference, for others it is.
//!

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::*;
use syn::*;


#[proc_macro_derive(ValueStruct)]
pub fn value_struct_macro(input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input).expect("failed to parse input");
    let span = Span::call_site();
    match item {
        Item::Struct(ref struct_item) => match struct_item.fields {
            Fields::Unnamed(ref unnamed_fields) if unnamed_fields.unnamed.len() == 1 => {
                let field = unnamed_fields.unnamed.first().unwrap();
                let struct_name = &struct_item.ident;
                let field_type = &field.ty;
                let parsed_field_type = parse_field_type(field_type);

                let type_dependent_functions =
                    create_type_dependent_functions(&field_type, parsed_field_type.as_ref());
                let type_dependent_impls =
                    create_dependent_impls(&struct_name, &field_type, parsed_field_type.as_ref());

                let output = quote! {
                    #[allow(dead_code)]
                    impl #struct_name {
                        #type_dependent_functions
                    }

                    #type_dependent_impls
                };

                output.into()
            }
            _ => Error::new(
                span,
                "ValueStruct works only on structs with one unnamed field",
            )
            .to_compile_error()
            .into(),
        },
        _ => Error::new(span, "ValueStruct works only on structs")
            .to_compile_error()
            .into(),
    }
}

enum ParsedType {
    StringType,
    ScalarType
}

#[inline]
fn parse_field_type(field_type: &Type) -> Option<ParsedType> {
    match field_type {
        Type::Path(ref path) => {
            let full_type_path: &String = &path
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<String>>()
                .join("::");

            match full_type_path.as_str() {
                "String" | "std::string::String" => Some(ParsedType::StringType),
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64"
                | "u128" | "usize" => Some(ParsedType::ScalarType),
                _ => None
            }
        }
        _ => None
    }
}

#[inline]
fn create_dependent_impls(
    struct_name: &Ident,
    field_type: &Type,
    parsed_field_type: Option<&ParsedType>,
) -> proc_macro2::TokenStream {
    match parsed_field_type {
        Some(ParsedType::ScalarType) => {
            quote! {
               impl std::convert::From<#field_type> for #struct_name {
                    fn from(value: #field_type) -> Self {
                        #struct_name(value)
                    }
               }
            }
        }
        Some(ParsedType::StringType) => {
            quote! {
                impl std::convert::From<std::string::String> for #struct_name {
                    fn from(value: String) -> Self {
                        #struct_name(value)
                    }
                }

                impl std::convert::From<&std::string::String> for #struct_name {
                    fn from(value: &String) -> Self {
                        #struct_name(value.clone())
                    }
                }

                impl std::convert::From<&str> for #struct_name {
                    fn from(value: &str) -> Self {
                        #struct_name(String::from(value))
                    }
                }
            }
        }
        _ => {
            quote! {
               impl std::convert::From<#field_type> for #struct_name {
                    fn from(value: #field_type) -> Self {
                        #struct_name(value)
                    }
               }

                impl std::convert::From<&#field_type> for #struct_name {
                    fn from(value: &#field_type) -> Self {
                        #struct_name(value.clone())
                    }
                }
            }
        }
    }
}

#[inline]
fn create_type_dependent_functions(
    field_type: &Type,
    parsed_field_type: Option<&ParsedType>,
) -> proc_macro2::TokenStream {
    match parsed_field_type {
        Some(ParsedType::ScalarType) => {
            quote! {
                #[inline]
                pub fn value(&self) -> #field_type {
                    self.0
                }
            }
        }
        _ => {
            quote! {
                #[inline]
                pub fn value(&self) -> &#field_type {
                    &self.0
                }
            }
        }
    }
}
