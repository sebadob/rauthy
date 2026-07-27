use proc_macro2::{Literal, TokenStream, TokenTree};
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Meta, MetaList, Type};

pub fn impl_from_pg_row(input: DeriveInput) -> proc_macro::TokenStream {
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut with_from_str = false;

    let body = match input.data {
        Data::Struct(data) => data
                .fields
                .iter()
                .filter_map(|field| {
                    let id = field.ident.as_ref()?;
                    let is_opt = is_field_ty_opt(&field.ty).unwrap_or(false);
                    let ch = ColumnHandler::from(field.attrs.as_slice());
                    let name = if let Some(name) = &ch.rename {
                        quote! {#name}
                    } else {
                        let name = id.to_string();
                        quote! {#name}
                    };

                    let ts = match ch.attr {
                        ColumnAttr::Flatten => {
                            quote! {#id: ::std::convert::TryFrom::try_from(&mut *row).unwrap(),}
                        }
                        ColumnAttr::FromI32 => {
                            // Note: the PG impl differs from hiqlite at this point.
                            // Postgres has a native `i32`, while hiqlite needs to downcast.
                            if is_opt {
                                quote! {
                                    #id: row.get::<_, Option<i32>>(#name).map(|i| i.into()),
                                }
                            } else {
                                quote! {
                                    #id: row.get::<_, i32>(#name).into(),
                                }
                            }
                        }
                        ColumnAttr::FromI64 => {
                            if is_opt {
                                quote! {#id: row.get::<_, Option<i64>>(#name).map(|i| i.into()),}
                            } else {
                                quote! {#id: row.get::<_, i64>(#name).into(),}
                            }
                        }
                        ColumnAttr::Parse => {
                            with_from_str = true;
                            if is_opt {
                                quote! {
                                #id: row.get::<_, Option<String>>(#name).map(|s| s.parse().unwrap()),
                            }
                            } else {
                                quote! {#id: row.get::<_, String>(#name).parse().unwrap(),}
                            }
                        }
                        ColumnAttr::FromString => {
                            if is_opt {
                                quote! {#id: row.get::<_, Option<String>>(#name).map(|s| s.into()),}
                            } else {
                                quote! {#id: row.get::<_, String>(#name).into(),}
                            }
                        }
                        ColumnAttr::None => {
                            quote! {#id: row.get(#name),}
                        }
                        ColumnAttr::Skip => quote! {#id: ::std::default::Default::default(),},
                    };
                    Some(ts)
                })
                .collect::<Vec<TokenStream>>(),
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    };

    let from_str = if with_from_str {
        quote! {use ::std::str::FromStr;}
    } else {
        quote! {}
    };

    quote! {
        impl #impl_generics ::std::convert::From<::tokio_postgres::Row> for #name #ty_generics #where_clause {
            #[inline]
            fn from(row: ::tokio_postgres::Row) -> Self {
                #from_str
                Self {
                    #(#body)*
                }
            }
        }
    }.into()
}

#[inline]
fn is_field_ty_opt(ty: &Type) -> Option<bool> {
    let Type::Path(ty) = ty else {
        return Some(false);
    };
    let mut iter = ty.path.segments.iter();

    let mut s = iter.next()?.ident.to_string();
    if s == "std" {
        s = iter.next()?.ident.to_string();
    }
    if s == "option" {
        s = iter.next()?.ident.to_string();
    }
    Some(s.as_str() == "Option")
}

struct ColumnHandler {
    rename: Option<Literal>,
    attr: ColumnAttr,
}

enum ColumnAttr {
    Flatten,
    FromI32,
    FromI64,
    Parse,
    FromString,
    None,
    Skip,
}

impl From<&[Attribute]> for ColumnHandler {
    fn from(attrs: &[Attribute]) -> Self {
        let mut rename: Option<Literal> = None;
        let mut attr = ColumnAttr::None;

        let do_panic = |idx: String| {
            panic!(
                r#"
Invalid syntax for '#[column]' - '{idx}' attribute, expected one of:

- flatten
- from_i32
- from_i64
- from_string
- parse
- rename = "my_column"
- skip
- rename may be combined with one of the from_* or parse attributes
"#
            )
        };

        for att in attrs {
            let Meta::List(MetaList { path, tokens, .. }) = &att.meta else {
                continue;
            };
            if let Some(seg) = path.segments.first()
                && seg.ident != "column"
            {
                continue;
            }

            let mut stream = tokens.clone().into_iter();
            let Some(tree) = stream.next() else {
                do_panic("missing first argument".to_string());
                break;
            };
            let value = tree.to_string();
            match value.as_str() {
                "flatten" => attr = ColumnAttr::Flatten,
                "skip" => attr = ColumnAttr::Skip,
                "rename" => {
                    if matches!(stream.next(), Some(TokenTree::Punct(p)) if p.as_char() == '=')
                        && let Some(TokenTree::Literal(lit)) = stream.next()
                    {
                        rename = Some(lit);

                        // check possibly following from_* attr
                        if let Some(tree) = stream.next() {
                            let TokenTree::Punct(p) = tree else {
                                do_panic("Invalid punctuation after rename".to_string());
                                break;
                            };
                            if p.as_char() != ',' {
                                do_panic(
                                    "Invalid punctuation after rename, expected ','".to_string(),
                                );
                            }
                            let Some(tree) = stream.next() else {
                                do_panic("Missing value after rename".to_string());
                                break;
                            };
                            let value = tree.to_string();
                            match value.as_str() {
                                "from_i32" => attr = ColumnAttr::FromI32,
                                "from_i64" => attr = ColumnAttr::FromI64,
                                "from_string" => attr = ColumnAttr::FromString,
                                "parse" => attr = ColumnAttr::Parse,
                                _ => do_panic(format!(
                                    "Invalid syntax for 'from_*' after 'rename': {value}"
                                )),
                            }
                        }
                    } else {
                        do_panic("cannot parse 'rename'".to_string());
                    }
                }
                other => {
                    match other {
                        "from_i32" => attr = ColumnAttr::FromI32,
                        "from_i64" => attr = ColumnAttr::FromI64,
                        "from_string" => attr = ColumnAttr::FromString,
                        "parse" => attr = ColumnAttr::Parse,
                        _ => do_panic(format!("Invalid syntax for 'from_*': {other}")),
                    }
                    if let Some(tree) = stream.next() {
                        let TokenTree::Punct(p) = tree else {
                            do_panic("Invalid punctuation after rename".to_string());
                            break;
                        };
                        if p.as_char() != ',' {
                            do_panic("Invalid punctuation after rename, expected ','".to_string());
                        }
                        let Some(tree) = stream.next() else {
                            do_panic("Missing value after rename".to_string());
                            break;
                        };
                        let value = tree.to_string();
                        if value != "rename" {
                            do_panic(format!(
                                "from_* attributes can only be combined with a rename, found: {value}"
                            ));
                            break;
                        }

                        if matches!(stream.next(), Some(TokenTree::Punct(p)) if p.as_char() == '=')
                            && let Some(TokenTree::Literal(lit)) = stream.next()
                        {
                            rename = Some(lit);
                        } else {
                            do_panic("cannot parse 'rename' after 'from_*'".to_string());
                        }
                    }
                }
            }
        }

        Self { rename, attr }
    }
}
