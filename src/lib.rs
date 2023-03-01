use proc_macro::TokenStream;
#[proc_macro_derive(ToSql)]
pub fn to_sql(input: TokenStream) -> TokenStream {
    println!("input: {}", input);
    TokenStream::new()
}
