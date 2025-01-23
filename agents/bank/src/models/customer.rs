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

#[cfg(test)]
pub fn get_random_customer() -> Customer {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    // let account_number: String = (0..11)
    //     .map(|_| rng.gen_range(0..10).to_string())
    //     .collect();
    Customer {
        uuid: Uuid::new_v4(),
        name: format!("customer-{}", rng.gen_range(1..=1000)),
        // account_number: account_number,
        balance: rng.gen_range(0..=1000),
    }
}
