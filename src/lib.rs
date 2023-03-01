mod to_sql_condition;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ToSqlCondition)]
pub fn to_condition(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // if you will debug, you can println input
    // it may be:
    /*
        DeriveInput {
        attrs: [],
        vis: Inherited,
        ident: Ident {
            ident: "Query",
            span: #0 bytes(50..55),
        },
        generics: Generics {
            //...
         },
        data: Struct(
            DataStruct {
                struct_token: Struct,
                fields: Named(
                    FieldsNamed {
                        brace_token: Brace,
                        named: [
                            Field {
                                attrs: [],
                                vis: Inherited,
                                ident: Some(
                                    Ident {
                                        ident: "id",
                                        span: #0 bytes(62..64),
                                    },
                                ),
                                colon_token: Some(
                                    Colon,
                                ),
                                ty: Path(
                                    TypePath {
                                        qself: None,
                                        path: Path {
                                            leading_colon: None,
                                            segments: [
                                                PathSegment {
                                                    ident: Ident {
                                                        ident: "Option",
                                                        span: #0 bytes(66..72),
                                                    },
                                                    arguments: AngleBracketed(
                                                        AngleBracketedGenericArguments {
                                                            colon2_token: None,
                                                            lt_token: Lt,
                                                            args: [
                                                                Type(
                                                                    Path(
                                                                        TypePath {
                                                                            qself: None,
                                                                            path: Path {
                                                                                leading_colon: None,
                                                                                segments: [
                                                                                    PathSegment {
                                                                                        ident: Ident {
                                                                                            ident: "i32",
                                                                                            span: #0 bytes(73..76),
                                                                                        },
                                                                                        arguments: None,
                                                                                    },
                                                                                ],
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
                                                            ],
                                                            gt_token: Gt,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                    },
                                ),
                            },
                            // ...
                            Comma,
                        ],
                    },
                ),
                semi_token: None,
            },
        ),
    }
         */
    // println!("{:#?}", input);
    to_sql_condition::impl_to_sql_condition(input).into()
}
