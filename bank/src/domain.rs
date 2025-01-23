use crate::{models::customer::Customer, usecase::BankRepository};
use shared::error::InterfaceError;
use uuid::Uuid;

/// Get the current balance of a customer
pub async fn get_balance(
    repo: &dyn BankRepository,
    uuid: Uuid,
) -> Result<Option<Customer>, InterfaceError> {
    let customer = repo.customers().get(&uuid).await?;
    match customer {
        Some(customer) => Ok(Some(customer)),
        None => Err(InterfaceError::MissingItem(uuid.to_string())),
    }
}

/// Create a customer account
pub async fn create_account(
    repo: &dyn BankRepository,
    customer: &Customer,
) -> Result<(), InterfaceError> {
    repo.customers().create(&customer).await
}

/// Order a new card for a customer
pub async fn order_card(repo: &dyn BankRepository, uuid: Uuid) -> Result<(), InterfaceError> {
    // Need to establish a connection with a network first
    todo!()
}

/// Authorize a transaction for a customer
/// Note: this doesn't actually perform a transaction
pub async fn authorize_transaction(
    repo: &dyn BankRepository,
    uuid: Uuid,
    amount: i32,
) -> Result<(), InterfaceError> {
    if amount <= 0 {
        return Err(InterfaceError::Other(
            "amount of transaction needs to be positive".to_string(),
        ));
    }

    let customer = repo.customers().get(&uuid).await?;
    if let None = customer {
        return Err(InterfaceError::MissingItem(uuid.to_string()));
    }

    // TODO: reserve the "authorized money"
    // and check amount > (balance - reserved)
    if amount > customer.unwrap().balance {
        return Err(InterfaceError::Other(
            "transaction refused: not enough balance".to_string(),
        ));
    }

    // TODO: if card is not yet activated, activate it
    Ok(())
}
