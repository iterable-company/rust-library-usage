use proc_macro2::Ident;
use syn::Attribute;
use syn::{GenericArgument, Path, PathArguments, Type, TypePath};

/// #[target(DomainStructName)]
/// Infrastructure 側の struct に上記のような attribute をつけて、変換先となるDomainのstructを指定することになっている
/// このメソッドはその Domain 側の Struct 名を返している
pub fn get_conversion_target(attrs: &[Attribute]) -> Ident {
    get_attribute_by_name(attrs, "target")
}

pub fn get_attribute_by_name(attrs: &[Attribute], name: &str) -> Ident {
    for attr in attrs {
        if attr.path().is_ident(name) {
            match attr.parse_args::<Ident>() {
                Ok(ident) => {
                    return ident;
                }
                Err(_) => panic!("{} attribute should contain an identifier", name),
            }
        }
    }
    panic!("{} attribute must be specified", name)
}

pub fn get_exclude_convert(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("exclude_convert") {
            return true;
        }
    }
    false
}

/// フィールドに convert_with が指定されている場合に、指定された変換関数を取得する
/// 指定されている場合は、Some(Ident) = 変換関数名
/// 指定されていない場合は None を返す
pub fn get_convert_with(attrs: &[Attribute]) -> Option<Ident> {
    for attr in attrs {
        if attr.path().is_ident("convert_with") {
            match attr.parse_args::<Ident>() {
                Ok(ident) => {
                    return Some(ident);
                }
                _ => return None,
            }
        }
    }
    None
}

/// フィールドの型が Option<T> であるか場合に、その型を取得する
/// 例）
/// 　フィールドの型が Option<T> である場合は、Some(Ident) = Some(Tの名前)　が返る
/// 　フィールドの型が Option ではない場合は、None が返る
pub fn get_option_type(ty: &Type) -> Option<&syn::Ident> {
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    {
        if let Some(segment) = segments.first() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(Type::Path(TypePath {
                        path:
                            Path {
                                segments: inner_segments,
                                ..
                            },
                        ..
                    }))) = args.args.first()
                    {
                        return inner_segments.first().map(|s| &s.ident);
                    }
                }
            }
        }
    }
    None
}

pub fn get_target_enum_names(attrs: &[Attribute]) -> Vec<Ident> {
    get_attibutes_by_name(attrs, "enum_target")
}

pub fn get_attibutes_by_name(attrs: &[Attribute], name: &str) -> Vec<Ident> {
    let mut ret: Vec<Option<Ident>> = vec![];
    for attr in attrs {
        if attr.path().is_ident(name) {
            let _ = attr.parse_nested_meta(|meta| {
                let ident = meta.path.get_ident().cloned();
                ret.push(ident);
                Ok(())
            });
        }
    }
    ret.into_iter().flatten().collect()
}
