pub mod memory;
pub mod rds;

use shared::ports::secondary::Repository;

use crate::models::{card::Card, customer::Customer};

pub trait BankRepository: Send + Sync {
    fn customers(&self) -> &dyn Repository<Customer>;

    fn cards(&self) -> &dyn Repository<Card>;
}
