mod row_to_domain;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::*;
use row_to_domain::{
    get_conversion_target, get_convert_with, get_exclude_convert, get_option_type, get_target_enum_names,
};

#[proc_macro_derive(ConvertTo, attributes(target, convert_with, exclude_convert))]
pub fn convert_to(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    eprintln!("ast: {:#?}", ast);
    match ast.data {
        Data::Struct(DataStruct { fields, .. }) => {
            let name = &ast.ident;
            let target_struct_name = get_conversion_target(&ast.attrs);

            let conversions = fields.iter().filter_map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                let exclude_convert = get_exclude_convert(&f.attrs);

                if exclude_convert {
                    return None;
                }

                let option_type = get_option_type(&f.ty);
                let is_option = option_type.is_some();

                let convert_with_fn = get_convert_with(&f.attrs).as_ref().map(|s| s.to_string()); // 行内でライフタイムがなくならいように一度実体として変数化しておく
                let convert_with_fn = convert_with_fn.as_deref();
                let convert = match convert_with_fn {
                    Some(fn_name) => {
                        let fn_ident = Ident::new(fn_name, Span::call_site());
                        quote! {
                            #fn_ident(item.#field_name)
                        }
                    }
                    _ => {
                        let is_bigdecimal = if let syn::Type::Path(ref p) = f.ty {
                            p.path.segments.last().unwrap().ident == "BigDecimal"
                        } else {
                            false
                        };

                        let is_option_bigdecimal = if let Some(inner_type) = option_type {
                            inner_type == "BigDecimal"
                        } else {
                            false
                        };

                        if is_bigdecimal {
                            quote! {
                                item.#field_name.to_f64().unwrap()
                            }
                        } else if is_option_bigdecimal {
                            quote! {
                                item.#field_name.map(|v| v.to_f64().unwrap())
                            }
                        } else if is_option {
                            quote! {
                                item.#field_name.map(|v| v.into())
                            }
                        } else {
                            quote! {
                                item.#field_name.into()
                            }
                        }
                    }
                };
                Some(convert)
            });

            let gen = quote! {
                impl From<#name> for #target_struct_name {
                    fn from(item: #name) -> Self {
                        Self::new(
                            #(#conversions,)*
                        )
                    }
                }
            };

            gen.into()
        }
        _ => panic!("only works with structs"),
    }
}

#[proc_macro_derive(EnumConvertFrom, attributes(enum_target))]
pub fn enum_mapper_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    //eprintln!("input: {:#?}", input);

    let data_enum = if let Data::Enum(de) = &input.data {
        de
    } else {
        panic!("This macro only works with enums");
    };

    let name: &Ident = &input.ident;
    let target_enum_names = get_target_enum_names(&input.attrs);
    let variants: Vec<_> = data_enum.variants.iter().map(|v| &v.ident).collect();

    let definitions = target_enum_names.into_iter().map(|target_enum_name| {
        quote! {
            impl From<#name> for #target_enum_name {
                fn from(variant: #name) -> Self {
                    match variant {
                        #(
                            #name::#variants => #target_enum_name::#variants,
                        )*
                    }
                }
            }
        }
    });

    quote! {
        #(#definitions)*
    }
    .into()
}
