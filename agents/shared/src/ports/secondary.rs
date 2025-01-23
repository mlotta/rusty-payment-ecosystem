use crate::{error::InterfaceError, Val};
use async_trait::async_trait;
use uuid::Uuid;

pub trait Repository<T>: Create<T> + Get<T> + Update<T> + List<T> + Delete<T>
where
    T: Val,
{
}

/// Create object trait
#[async_trait]
pub trait Create<T>
where
    T: Val,
{
    async fn create(&self, item: &T) -> Result<(), InterfaceError>;
}

/// Get object trait
#[async_trait]
pub trait Get<T>
where
    T: Val,
{
    async fn get(&self, id: &Uuid) -> Result<Option<T>, InterfaceError>;
}

/// Delete object trait
#[async_trait]
pub trait Delete<T>
where
    T: Val,
{
    async fn delete(&self, id: &Uuid) -> Result<(), InterfaceError>;
}

/// Update object trait
#[async_trait]
pub trait Update<T>
where
    T: Val,
{
    async fn update(&self, item: &T) -> Result<(), InterfaceError>;
}

/// Get object range trait
#[async_trait]
pub trait List<T>
where
    T: Val,
{
    async fn list(&self) -> Result<Vec<T>, InterfaceError>;
}
