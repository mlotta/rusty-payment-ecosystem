//! Customer domain entity

use serde::{Deserialize, Serialize};
use shared::sql_macros::struct_to_sql;
use shared::usecase::rds::GetFieldsAsParams;
use shared::QuerySet;
use uuid::Uuid;

/// Customer
#[derive(Deserialize, Serialize)]
#[struct_to_sql]
pub struct Customer {
    #[serde(default = "uuid::Uuid::new_v4")]
    uuid: Uuid,
    // #[serde(default)]
    // account_number: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    balance: i32, //TODO
                  // #[serde(default)]
                  // created_at:
}
