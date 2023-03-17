# to_sql_condition
"To_sql_condition" is a convenient derived macro that generates SQL query conditions, reducing the amount of manual coding required. However, it currently has limited capabilities and can only support a few features, such as WHERE and LIMIT.

## useages

table

```sql
CREATE TABLE `od_commands` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `name` char(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci
```

Perhaps we want to use query commands with certain files. In Rust, we could create a struct to handle this.

```rust
use core::option;
use to_sql_condition::ToSqlCondition;

#[derive(ToSqlCondition, Debug)]
struct Query {
    id: ::core::option::Option<i32>,
    name: option::Option<String>,
    offset: Option<u32>,
    limit: Option<u32>,
}

#[derive(ToSqlCondition, Debug)]
struct QueryNoOption {
    id: i32,
    name: String,
    offset: u32,
    limit: u32,
}

fn main() {
    let (q1, q2, q3, q4, q5) = (
        Query {
            id: Some(1),
            name: Some("tom".to_string()),
            offset: None,
            limit: Some(10),
        },
        Query {
            id: None,
            name: None,
            offset: None,
            limit: None,
        },
        Query {
            id: Some(1),
            name: None,
            offset: Some(1),
            limit: None,
        },
        Query {
            id: Some(1),
            name: None,
            offset: Some(1),
            limit: Some(1),
        },
        Query {
            id: Some(1),
            name: Some("tom".to_string()),
            offset: None,
            limit: None,
        },
    );
    assert_eq!(
        q1.to_sql_condition(),
        " WHERE id = 1 AND name = 'tom' LIMIT 10".to_string()
    );
    assert_eq!(q2.to_sql_condition(), "".to_string());
    assert_eq!(q3.to_sql_condition(), " WHERE id = 1 OFFSET 1".to_string());
    assert_eq!(
        q4.to_sql_condition(),
        " WHERE id = 1 OFFSET 1 LIMIT 1".to_string()
    );
    assert_eq!(
        q5.to_sql_condition(),
        " WHERE id = 1 AND name = 'tom'".to_string()
    );

    let q = QueryNoOption {
        id: 1,
        name: "tom".to_string(),
        offset: 1,
        limit: 1,
    };

    assert_eq!(
        q.to_sql_condition(),
        " WHERE id = 1 AND name = 'tom' OFFSET 1 LIMIT 1"
    );
}
```

