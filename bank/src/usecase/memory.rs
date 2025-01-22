use crate::models::{
    card::Card,
    customer::Customer,
};
use crate::usecase::BankRepository;
use shared::ports::secondary::Repository;
use shared::usecase::memory::{HasUuid, InMemoryRepository};

impl HasUuid for Customer{
    fn get_uuid(&self) -> uuid::Uuid {
        self.uuid
    }
}

impl HasUuid for Card{
    fn get_uuid(&self) -> uuid::Uuid {
        self.uuid
    }
}

pub struct BankMemoryRepository {
    customers: InMemoryRepository<Customer>,
    cards: InMemoryRepository<Card>,
}

impl BankMemoryRepository {
    pub fn new() -> Self {
        let customers: InMemoryRepository<Customer> = InMemoryRepository::new();
        let cards: InMemoryRepository<Card> = InMemoryRepository::new();
        Self {
            customers,
            cards,
        }
    }
}

impl BankRepository for BankMemoryRepository {
    fn customers(&self) -> &dyn Repository<Customer> {
        &self.customers
    }

    fn cards(&self) -> &dyn Repository<Card> {
        &self.cards
    }
}
