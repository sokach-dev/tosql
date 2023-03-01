use tosql::ToSql;

#[derive(ToSql, Debug)]
struct Foo {
    aoo: i32,
    boo: String,
    coo: Option<String>,
    doo: Vec<String>,
}

fn main() {
    let foo = Foo {
        aoo: 1,
        boo: "hello".to_string(),
        coo: Some("world".to_string()),
        doo: vec!["foo".to_string(), "bar".to_string()],
    };
    println!("foo: {:?}", foo);
}
