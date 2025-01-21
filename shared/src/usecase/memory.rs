//! In Memory implementation of a Repository
use crate::ports::secondary::{Create, Delete, Get, List, Repository, Update};
use crate::{error::InterfaceError, Val};
use async_trait::async_trait;
use std::{collections::HashMap, sync::RwLock};
use uuid::Uuid;

pub trait HasUuid {
    fn get_uuid(&self) -> Uuid;
}

#[derive(Default)]
pub struct InMemoryRepository<T>
where
    T: Val + HasUuid,
{
    data: RwLock<HashMap<Uuid, T>>,
}

impl<T> InMemoryRepository<T>
where
    T: Val + HasUuid,
{
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait]
impl<T> Create<T> for InMemoryRepository<T>
where
    T: Val + HasUuid,
{
    async fn create(&self, item: &T) -> Result<(), InterfaceError> {
        self.data
            .write()
            .unwrap()
            .insert(item.get_uuid().clone(), item.clone());
        Ok(())
    }
}

#[async_trait]
impl<T> Get<T> for InMemoryRepository<T>
where
    T: Val + HasUuid,
{
    async fn get(&self, id: &Uuid) -> Result<Option<T>, InterfaceError> {
        Ok(self.data.read().unwrap().get(id).cloned())
    }
}

#[async_trait]
impl<T> Delete<T> for InMemoryRepository<T>
where
    T: Val + HasUuid,
{
    async fn delete(&self, id: &Uuid) -> Result<(), InterfaceError> {
        self.data.write().unwrap().remove(id);
        Ok(())
    }
}

#[async_trait]
impl<T> Update<T> for InMemoryRepository<T>
where
    T: Val + HasUuid,
{
    async fn update(&self, item: &T) -> Result<(), InterfaceError> {
        self.data
            .write()
            .unwrap()
            .insert(item.get_uuid().clone(), item.clone());
        Ok(())
    }
}

#[async_trait]
impl<T> List<T> for InMemoryRepository<T>
where
    T: Val + HasUuid,
{
    async fn list(&self) -> Result<Vec<T>, InterfaceError> {
        Ok(self
            .data
            .read()
            .unwrap()
            .iter()
            .map(|(_, v)| v.clone())
            .collect())
    }
}

#[async_trait]
impl<T> Repository<T> for InMemoryRepository<T> where T: Val + HasUuid {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    // Define structures
    #[derive(Default, Debug, Clone, Deserialize, PartialEq, Serialize)]
    struct Item1 {
        uuid: Uuid,
        field1: i32,
    }

    impl HasUuid for Item1 {
        fn get_uuid(&self) -> Uuid {
            self.uuid
        }
    }

    // Gen item
    fn gen_item() -> Item1 {
        Item1 {
            uuid: Uuid::new_v4(),
            field1: 3,
        }
    }

    #[tokio::test]
    async fn test_new() -> Result<(), InterfaceError> {
        // GIVEN an empty repository
        let repo: InMemoryRepository<Item1> = InMemoryRepository::new();

        // WHEN we get the length of all items
        // THEN we get 0
        assert_eq!(repo.data.read().unwrap().len(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_all_empty() -> Result<(), InterfaceError> {
        // GIVEN an empty repo
        let repo: InMemoryRepository<Item1> = InMemoryRepository::new();

        // WHEN we get all items
        let all = repo.list().await?;

        // THEN we get an empty list
        assert_eq!(all.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_all() -> Result<(), InterfaceError> {
        // GIVEN a repo with two items
        let item1 = gen_item();
        let item2 = gen_item();
        let repo: InMemoryRepository<Item1> = InMemoryRepository::new();
        {
            let mut data = repo.data.write().unwrap();
            data.insert(item1.uuid.clone(), item1.clone());
            data.insert(item2.uuid.clone(), item2.clone());
        }

        // WHEN we get all ITEM1_s
        let all = repo.list().await?;

        // THEN we get the ITEM1_s
        assert_eq!(all.len(), 2);
        assert!(all.contains(&item1));
        assert!(all.contains(&item2));

        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> Result<(), InterfaceError> {
        // GIVEN a repo with one item
        let item = gen_item();
        let repo: InMemoryRepository<Item1> = InMemoryRepository::new();
        {
            let mut data = repo.data.write().unwrap();
            data.insert(item.uuid.clone(), item.clone());
        }

        // WHEN deleting the item
        repo.delete(&item.uuid).await?;

        // THEN the length of the repo is 0
        assert_eq!(repo.data.read().unwrap().len(), 0);
        // AND the item is not returned
        assert_eq!(repo.get(&item.uuid).await?, None);

        Ok(())
    }

    #[tokio::test]
    async fn test_get() -> Result<(), InterfaceError> {
        // GIVEN a repo with an item
        let item = gen_item();
        let repo: InMemoryRepository<Item1> = InMemoryRepository::new();
        {
            let mut data = repo.data.write().unwrap();
            data.insert(item.uuid.clone(), item.clone());
        }

        // WHEN getting the product
        let item_from_dataset = repo.get(&item.uuid).await?;

        // THEN the product is returned
        assert_eq!(item_from_dataset, Some(item));

        Ok(())
    }

    #[tokio::test]
    async fn test_update() -> Result<(), InterfaceError> {
        // GIVEN an empty repo and an item
        let item = gen_item();
        let repo: InMemoryRepository<Item1> = InMemoryRepository::new();

        // WHEN inserting a product
        repo.update(&item).await?;

        // THEN the length of the repo is 1
        assert_eq!(repo.data.read().unwrap().len(), 1);
        // AND the item is returned
        assert_eq!(repo.get(&item.uuid).await?, Some(item));

        Ok(())
    }

    #[tokio::test]
    async fn test_create() -> Result<(), InterfaceError> {
        // GIVEN an empty repo and an item
        let item = gen_item();
        let repo: InMemoryRepository<Item1> = InMemoryRepository::new();

        // WHEN inserting a product
        repo.create(&item).await?;

        // THEN the length of the repo is 1
        assert_eq!(repo.data.read().unwrap().len(), 1);
        // AND the item is returned
        assert_eq!(repo.get(&item.uuid).await?, Some(item));

        Ok(())
    }
}
