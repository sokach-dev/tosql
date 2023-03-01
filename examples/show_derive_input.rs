use to_sql_condition::ToSqlCondition;

#[derive(ToSqlCondition, Debug)]
struct Query {
    id: Option<i32>,
    name: Option<String>,
    limit: Option<u32>,
}

#[derive(ToSqlCondition, Debug)]
struct QueryNoOption {
    id: i32,
    name: String,
    limit: u32,
}

fn main() {
    let (q1, q2, q3, q4, q5) = (
        Query {
            id: Some(1),
            name: Some("tom".to_string()),
            limit: Some(10),
        },
        Query {
            id: None,
            name: None,
            limit: None,
        },
        Query {
            id: Some(1),
            name: None,
            limit: None,
        },
        Query {
            id: Some(1),
            name: None,
            limit: Some(1),
        },
        Query {
            id: Some(1),
            name: Some("tom".to_string()),
            limit: None,
        },
    );
    assert_eq!(
        q1.to_sql_condition(),
        " WHERE `id` = 1 AND `name` = 'tom' LIMIT 10".to_string()
    );
    assert_eq!(q2.to_sql_condition(), "".to_string());
    assert_eq!(q3.to_sql_condition(), " WHERE `id` = 1".to_string());
    assert_eq!(q4.to_sql_condition(), " WHERE `id` = 1 LIMIT 1".to_string());
    assert_eq!(
        q5.to_sql_condition(),
        " WHERE `id` = 1 AND `name` = 'tom'".to_string()
    );

    let q = QueryNoOption {
        id: 1,
        name: "tom".to_string(),
        limit: 1,
    };

    assert_eq!(
        q.to_sql_condition(),
        " WHERE `id` = 1 AND `name` = 'tom' LIMIT 1"
    );
}
