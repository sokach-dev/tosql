# to_sql_condition
to_sql_condition is a simple derived macro that generates conditions for SQL query statements to reduce coding. Currently, it only supports a limited set of features, including WHERE and LIMIT.

## useages

table

```sql
CREATE TABLE `od_commands` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `name` char(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  `ext` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci
```

maybe we wan't query commands with some files, in rust, we may give a struct.

```rust
use to_sql_condition::ToSqlCondition;

#[derive(ToSqlCondition)]
struct Query {
    id: Option<i64>,
    name: Option<String>,
    ext: Option<String>,
    limit: Option<i32>,
}

fn main() {
    let (q, q2, q3) = (
        Query{id: None, name: Some("tom"), limit: Some(1)},
        Query{id: None, name: None, limit: Some(10)},
        Query{id: Some(2), name: None, limit: None},
    );
    assert_eq!(q.to_sql_condition(), " WHERE `name` = 'tom' AND LIMIT 1");
    assert_eq!(q2.to_sql_condition(), " LIMIT 10");
    assert_eq!(q3.to_sql_condition(), " WHERE `id` = 2");
}
```

