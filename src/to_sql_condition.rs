use proc_macro::TokenStream;
use quote::quote;
use syn::{
    AngleBracketedGenericArguments, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed,
    GenericArgument, PathArguments::AngleBracketed, Type, TypePath,
};

pub fn impl_to_sql_condition(input: DeriveInput) -> TokenStream {
    let name = input.ident;

    // get the fields of the struct
    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { named, .. }),
        ..
    }) = input.data
    {
        named
    } else {
        panic!("Only structs with named fields are supported");
    };

    // traverse the fields and generate the code
    let mut limit_clause = quote! {};
    let mut where_clause = quote! {};

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let (is_option, is_number) = field_is_option_and_get_type(&field);
        if is_option {
            if field_name == "limit" {
                limit_clause = quote! {
                    #limit_clause
                    if let Some(field_value) = &self.#field_name {
                        limit_clause.push_str(&format!(" LIMIT {}", field_value));
                    }
                };
            } else {
                where_clause = quote! {
                    #where_clause
                    if let Some(field_value) = &self.#field_name {
                        if #is_number {
                            where_clause.push_str(&format!(" AND `{}` = {}", stringify!(#field_name), field_value));
                        } else {
                            where_clause.push_str(&format!(" AND `{}` = '{}'", stringify!(#field_name), field_value));
                        }
                    }
                };
            }
        } else {
            if field_name == "limit" {
                limit_clause = quote! {
                    #limit_clause
                    limit_clause.push_str(&format!(" LIMIT {}", self.#field_name));
                };
            } else {
                where_clause = quote! {
                    #where_clause
                    if #is_number {
                        where_clause.push_str(&format!(" AND `{}` = {}", stringify!(#field_name), self.#field_name));
                    } else {
                    where_clause.push_str(&format!(" AND `{}` = '{}'", stringify!(#field_name), self.#field_name));
                    }
                };
            }
        }
    }

    let output = quote! {
        impl #name {
            pub fn to_sql_condition(&self) -> String {
                let mut sql = String::new();
                let mut where_clause = String::new();
                let mut limit_clause = String::new();
                // generate the where clause
                #where_clause

                if !where_clause.is_empty() {
                    sql.push_str(" WHERE ");
                    // remove the first " AND "
                    sql.push_str(&where_clause[5..]);
                }

                // generate the limit clause
                #limit_clause
                sql.push_str(&limit_clause);

                sql
            }
        }
    };

    output.into()
}

// judge whether the field is Option
fn field_is_option_and_get_type(field: &Field) -> (bool, bool) {
    if let syn::Type::Path(TypePath { path, .. }) = &field.ty {
        if let Some(segment) = path.segments.last() {
            if segment.ident == "Option" {
                if let AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
                    &segment.arguments
                {
                    if let Some(GenericArgument::Type(Type::Path(TypePath { path, .. }))) =
                        args.first()
                    {
                        if let Some(segment) = path.segments.first() {
                            return (true, field_is_number(&segment.ident.to_string()));
                        }
                    }
                }
            } else {
                return (false, field_is_number(&segment.ident.to_string()));
            }
        }
    }
    return (false, field_is_number("unknown"));
}

// judge whether is a number
fn field_is_number(field_type: &str) -> bool {
    if field_type == "i8"
        || field_type == "i16"
        || field_type == "i32"
        || field_type == "i64"
        || field_type == "i128"
        || field_type == "u8"
        || field_type == "u16"
        || field_type == "u32"
        || field_type == "u64"
        || field_type == "u128"
        || field_type == "f32"
        || field_type == "f64"
    {
        return true;
    }
    return false;
}
