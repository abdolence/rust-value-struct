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

                // let type_dependent_functions =
                //     create_type_dependent_functions(&field_type, parsed_field_type.as_ref());
                let type_dependent_impls =
                    create_dependent_impls(&struct_name, &field_type, parsed_field_type.as_ref());

                let output = quote! {
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
    ScalarType,
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

    let all_types_base_impl = quote! {

        impl ValueStruct for #struct_name {
            type ValueType = #field_type;

            #[inline]
            fn value(&self) -> &Self::ValueType {
                &self.0
            }
        }

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
    };

    match parsed_field_type {
        Some(ParsedType::StringType) => {
            quote! {
                impl std::convert::From<&str> for #struct_name {
                    fn from(value: &str) -> Self {
                        #struct_name(String::from(value))
                    }
                }

                impl std::str::FromStr for #struct_name {
                    type Err = std::string::ParseError;

                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        Ok(#struct_name(s.into()))
                    }
                }

                impl std::convert::AsRef<str> for #struct_name {
                    fn as_ref(&self) -> &str {
                        self.value().as_str()
                    }
                }

                #all_types_base_impl

            }
        }
        _ => {
            quote! {
                #all_types_base_impl
            }
        }
    }
}