use anyhow::Error;
use async_trait::async_trait;
use crate::{Key, Val, PrimaryKey};

/// Create object trait
#[async_trait]
pub trait Create<T, K> 
where
    T: PrimaryKey<K> + Val,
    K: Key
{
    async fn create(&self, item: &T) -> Result<(), Error>;
}

/// Get object trait
#[async_trait]
pub trait Get<T, K> 
where
    T: PrimaryKey<K> + Val,
    K: Key
{
    async fn get(&self, id: &K) -> Result<Option<T>, Error>;
}

/// Delete object trait
#[async_trait]
pub trait Delete<T, K> 
where
    T: PrimaryKey<K> + Val,
    K: Key
{
    async fn delete(&self, id: &K) -> Result<(), Error>;
}

/// Update object trait
#[async_trait]
pub trait Update<T, K> 
where
    T: PrimaryKey<K> + Val,
    K: Key
{
    async fn update(&self, item: &T) -> Result<(), Error>;
}

/// Get object range trait
#[async_trait]
pub trait GetAll<T, K> 
where
    T: Val + PrimaryKey<K>,
    K: Key
{
    async fn all(&self) -> Result<Vec<T>, Error>;
}
