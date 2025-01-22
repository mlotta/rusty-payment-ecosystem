use std::collections::HashMap;

use sql_macros::struct_to_sql;
// use std::marker::PhantomData;
use uuid::Uuid;

/// Redefining the trait here for testing
trait QuerySet<T> {
    /// Table name
    fn table(&self) -> String;

    /// SQL query to create a new table
    fn create_table(&self) -> String;

    /// SQL query to drop a table
    fn drop_table(&self) -> String;

    /// SQL query to delete an object by field (prepared)
    fn delete(&self, field_name: &str) -> String;

    /// SQL query to get an object by field (prepared)
    fn get(&self, field_name: &str) -> String;

    /// SQL query to create an object (prepared)
    fn create(&self) -> String;

    /// SQL query to update an object (prepared)
    fn update(&self) -> String;

    /// SQL query to list all items
    fn list(&self) -> String;
}

/// Build a Vec<SqlParameter> to use in ExecuteStatementBuilder::set_parameters.
/// from an items fields
trait GetFieldsAsParams {
    fn get_fields_as_params(&self) -> Option<Vec<aws_sdk_rdsdata::types::SqlParameter>>;
}

#[struct_to_sql]
struct BaseModel {
    name: String,
    id: i32,
    uuid: Uuid,
}

#[test]
fn test_base() {
    use pretty_assertions::assert_eq;
    let item = BaseModel {
        name: "abc".to_string(),
        id: 5,
        uuid: Uuid::parse_str("0829b81a-f86e-4411-870a-ca16e6b73189").unwrap(),
    };

    let queryset: BaseModelQuerySet<BaseModel> = BaseModel::queryset();

    assert_eq!(item.name, "abc".to_string());
    assert_eq!(item.id, 5);

    assert_eq!(queryset.table(), "BaseModel".to_string());
    assert_eq!(
        queryset.create_table(),
        "CREATE TABLE IF NOT EXISTS BaseModel (name VARCHAR(255), id INTEGER, uuid UUID)"
            .to_string()
    );
    assert_eq!(
        queryset.drop_table(),
        "DROP TABLE IF EXISTS BaseModel".to_string()
    );
    assert_eq!(
        queryset.delete("id"),
        "DELETE FROM BaseModel WHERE id = :id".to_string()
    );
    assert_eq!(
        queryset.get("id"),
        "SELECT * FROM BaseModel WHERE id = :id".to_string()
    );
    assert_eq!(queryset.list(), r#"SELECT * FROM BaseModel"#.to_string());
    assert_eq!(
        queryset.create(),
        r#"INSERT INTO BaseModel (name, id, uuid) VALUES (:name, :id, :uuid)"#.to_string()
    );
    assert_eq!(
        queryset.update(),
        r#"UPDATE BaseModel SET name = :name, id = :id WHERE uuid = :uuid"#.to_string()
    );

    let mut ground = HashMap::new();
    ground.insert("name", "abc");
    ground.insert("id", "5");
    ground.insert("uuid", "0829b81a-f86e-4411-870a-ca16e6b73189");
    let fields_as_params = item.get_fields_as_params().unwrap();
    for param in fields_as_params {
        let name = param.name().unwrap();
        let _ = match param.value().unwrap() {
            aws_sdk_rdsdata::types::Field::StringValue(s) => {
                assert_eq!(s.to_string(), *ground.get(name).unwrap())
            }
            aws_sdk_rdsdata::types::Field::LongValue(i) => {
                let g: i64 = (*ground.get(name).unwrap()).parse::<i64>().unwrap();
                assert_eq!(*i, g);
            }
            _ => unimplemented!(),
        };
    }
}

// This should not compile

// #[struct_to_sql]
// pub struct ModelPhantomData<T> {
//     name: String,
//     _unused: PhantomData<T>
// }

// #[struct_to_sql]
// pub struct ModelUnimplemented {
//     name: String,
//     id: u32
// }

// #[test]
// fn test_phantom_data() {
//     let model = ModelPhantomData {
//         name: "abc".to_string(),
//         _unused: PhantomData
//     };

// }

// #[test]
// fn test_not_implemented() {
//     let model = ModelUnimplemented {
//         name: "abc".to_string(),
//         id: 5,
//     };
// }
