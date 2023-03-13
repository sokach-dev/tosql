mod to_sql_condition;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ToSqlCondition)]
pub fn to_condition(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // if you will debug, you can println input
    // println!("{:#?}", input);
    to_sql_condition::impl_to_sql_condition(input).into()
}
