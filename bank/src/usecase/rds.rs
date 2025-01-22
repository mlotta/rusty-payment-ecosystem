use crate::models::{
    card::{Card, CardQuerySet},
    customer::{Customer, CustomerQuerySet},
};
use crate::usecase::BankRepository;
use aws_config::SdkConfig;
use shared::ports::secondary::Repository;
use shared::settings::RdsSettings;
use shared::{rds_client::RdsClient, usecase::rds::RdsRepository};

use std::sync::Arc;

pub struct BankRdsRepository {
    customers: RdsRepository<Customer, CustomerQuerySet<Customer>>,
    cards: RdsRepository<Card, CardQuerySet<Card>>,
}

impl BankRdsRepository {
    pub fn new(settings: &RdsSettings, sdk_config: &SdkConfig) -> Self {
        let client = Arc::new(RdsClient::new(settings, sdk_config));
        let customer_queryset: Box<CustomerQuerySet<Customer>> = Box::new(Customer::queryset());
        let customers = RdsRepository::new(Arc::clone(&client), customer_queryset);

        let card_queryset: Box<CardQuerySet<Card>> = Box::new(Card::queryset());
        let cards = RdsRepository::new(Arc::clone(&client), card_queryset);

        BankRdsRepository { customers, cards }
    }
}

impl BankRepository for BankRdsRepository {
    fn customers(&self) -> &dyn Repository<Customer> {
        &self.customers
    }

    fn cards(&self) -> &dyn Repository<Card> {
        &self.cards
    }
}
