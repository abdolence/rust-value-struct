use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, Ident, Type};

#[proc_macro_derive(ValueStruct)]
pub fn value_struct_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match value_struct_impls(&input) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn value_struct_impls(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let Data::Struct(data) = &input.data else {
        return Err(Error::new_spanned(
            &input.ident,
            "ValueStruct works only on structs",
        ));
    };

    match &data.fields {
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            let field_type = &fields.unnamed[0].ty;
            Ok(create_dependent_impls(
                &input.ident,
                field_type,
                parse_field_type(field_type),
            ))
        }
        Fields::Unit => Err(Error::new_spanned(
            &input.ident,
            "ValueStruct works only on structs with one unnamed field",
        )),
        fields => Err(Error::new_spanned(
            fields,
            "ValueStruct works only on structs with one unnamed field",
        )),
    }
}

enum ParsedType {
    StringType,
    ScalarType,
}

const SCALAR_TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
];

/// Classifies the field type by its path as written, since a derive macro
/// cannot resolve types. Changing what is recognised here changes which impls
/// are generated, which can collide with impls users have written by hand.
fn parse_field_type(field_type: &Type) -> Option<ParsedType> {
    let Type::Path(type_path) = field_type else {
        return None;
    };
    let segments = &type_path.path.segments;
    let is_path = |expected: &[&str]| {
        segments.len() == expected.len() && segments.iter().zip(expected).all(|(s, e)| s.ident == e)
    };

    if is_path(&["String"]) || is_path(&["std", "string", "String"]) {
        Some(ParsedType::StringType)
    } else if SCALAR_TYPES.iter().any(|scalar| is_path(&[scalar])) {
        Some(ParsedType::ScalarType)
    } else {
        None
    }
}

/// Every path in the generated code is absolute, so that names in the user's
/// module (a local `Result<T>` alias, for example) cannot capture it.
/// `ValueStruct` is the exception: it stays unqualified so that users of
/// `rvs_derive` alone can supply their own trait of that name.
fn create_dependent_impls(
    struct_name: &Ident,
    field_type: &Type,
    parsed_field_type: Option<ParsedType>,
) -> proc_macro2::TokenStream {
    let all_types_base_impl = quote! {
        impl #struct_name {
            pub const fn new(value: #field_type) -> Self {
                Self(value)
            }
        }

        #[automatically_derived]
        impl ValueStruct for #struct_name {
            type ValueType = #field_type;

            #[inline]
            fn value(&self) -> &Self::ValueType {
                &self.0
            }

            #[inline]
            fn into_value(self) -> Self::ValueType {
                self.0
            }
        }

        #[automatically_derived]
        impl ::std::convert::From<#field_type> for #struct_name {
            fn from(value: #field_type) -> Self {
                Self(value)
            }
        }

        #[automatically_derived]
        impl ::std::convert::From<&#field_type> for #struct_name {
            fn from(value: &#field_type) -> Self {
                Self(::std::clone::Clone::clone(value))
            }
        }
    };

    let display_impl = quote! {
        #[automatically_derived]
        impl ::std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
    };

    match parsed_field_type {
        Some(ParsedType::ScalarType) => quote! {
            #all_types_base_impl
            #display_impl
        },
        Some(ParsedType::StringType) => quote! {
            #all_types_base_impl

            #[automatically_derived]
            impl ::std::convert::From<&str> for #struct_name {
                fn from(value: &str) -> Self {
                    Self(::std::string::String::from(value))
                }
            }

            #[automatically_derived]
            impl ::std::str::FromStr for #struct_name {
                type Err = ::std::string::ParseError;

                fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                    ::std::result::Result::Ok(Self(::std::string::String::from(s)))
                }
            }

            #[automatically_derived]
            impl ::std::convert::AsRef<str> for #struct_name {
                fn as_ref(&self) -> &str {
                    ::std::string::String::as_str(&self.0)
                }
            }

            #display_impl
        },
        None => all_types_base_impl,
    }
}
