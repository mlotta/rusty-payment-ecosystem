use crate::ports::secondary::{Create, Get, Update, Delete, GetAll};
use std::{collections::HashMap, sync::RwLock};
use async_trait::async_trait;
use anyhow::Error;
use crate::{Key, Val, PrimaryKey};


#[derive(Default)]
pub struct InMemory<T, K>
where
    T: PrimaryKey<K> + Val,
    K: Key
{
        data: RwLock<HashMap<K, T>>
}

impl<T, K> InMemory<T, K> 
where 
    T: Val + PrimaryKey<K>,
    K: Key
{
    pub fn new() -> Self {
        Default::default()
    }
}


#[async_trait]
impl<T, K> Create<T, K> for InMemory<T, K> 
where
    T: Val + PrimaryKey<K>,
    K: Key
{
    async fn create(&self, item: &T) -> Result<(), Error>{
        self.data
            .write()
            .unwrap()
            .insert(item.get_pk().clone(), item.clone());
        Ok(())
    }
}

#[async_trait]
impl<T, K> Get<T, K> for InMemory<T, K> 
where
    T: Val + PrimaryKey<K>,
    K: Key
{
    async fn get(&self, id: &K) -> Result<Option<T>, Error>{
        Ok(self.data.read().unwrap().get(id).cloned())

    }

}


#[async_trait]
impl<T, K> Delete<T, K> for InMemory<T, K> 
where 
    K: Key,
    T: Val + PrimaryKey<K>
{
    async fn delete(&self, id: &K) -> Result<(), Error> {
        self.data.write().unwrap().remove(id);
        Ok(())
    }
}


#[async_trait]
impl<T, K> Update<T, K> for InMemory<T, K> 
where 
    K: Key,
    T: Val + PrimaryKey<K>
{
    async fn update(&self, item: &T) -> Result<(), Error> {
        self.data
            .write()
            .unwrap()
            .insert(item.get_pk().clone(), item.clone());
        Ok(())
    }
}

#[async_trait]
impl<T, K> GetAll<T, K> for InMemory<T, K> 
where 
    K: Key,
    T: Val + PrimaryKey<K>
{
    async fn all(&self) -> Result<Vec<T>, Error>{
        Ok(self
            .data
            .read()
            .unwrap()
            .iter()
            .map(|(_, v)| v.clone())
            .collect()
        )

    }
}





#[cfg(test)]
mod tests {
    use super::*;
    // use uuid::Uuid;
    use serde::{Deserialize, Serialize};

    // Define structures
    // pk: i32
    #[derive(Default, Debug, Clone, Deserialize, PartialEq, Serialize)]
    struct Item1 {
        pk: i32,
        field1: &'static str,
    }

    impl PrimaryKey<i32> for Item1 {
        fn get_pk(&self) -> i32 {
            self.pk
        }
    }

    // pk: uuid
    // struct Item2 {
    //     pk: Uuid,
    //     field1: &'static str,
    // }


    // impl PrimaryKey<Uuid> for Item2 {
    //     fn get_pk(&self) -> Uuid {
    //         self.pk
    //     }
    // }

    // Define constants
    const ITEM1_0: Item1 = Item1 {
        pk: 0,
        field1: "abc"
    };

    const ITEM1_1: Item1 = Item1 {
        pk: 1,
        field1: "bcd"
    };


    #[tokio::test]
    async fn test_new() -> Result<(), Error> {
        // GIVEN an empty repository
        let repo: InMemory<Item1, i32> = InMemory::new();

        // WHEN we get the length of all items
        // THEN we get 0
        assert_eq!(repo.data.read().unwrap().len(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_all_empty() -> Result<(), Error> {
        // GIVEN an empty repo
        let repo: InMemory<Item1, i32> = InMemory::new();

        // WHEN we get all items
        let all = repo.all().await?;

        // THEN we get an empty list
        assert_eq!(all.len(), 0);

        Ok(())
    }



    #[tokio::test]
    async fn test_all() -> Result<(), Error> {
        // GIVEN a repo with two items
        let repo: InMemory<Item1, i32> = InMemory::new();
        {
            let mut data = repo.data.write().unwrap();
            data.insert(ITEM1_0.get_pk().clone(), ITEM1_0.clone());
            data.insert(ITEM1_1.get_pk().clone(), ITEM1_1.clone());
        }

        // WHEN we get all ITEM1_s
        let all = repo.all().await?;

        // THEN we get the ITEM1_s
        assert_eq!(all.len(), 2);
        assert!(all.contains(&ITEM1_0));
        assert!(all.contains(&ITEM1_1));

        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> Result<(), Error> {
        // GIVEN a repo with one item
        let repo: InMemory<Item1, i32> = InMemory::new();
        {
            let mut data = repo.data.write().unwrap();
            data.insert(ITEM1_0.get_pk().clone(), ITEM1_0.clone());
        }

        // WHEN deleting the item
        repo.delete(&ITEM1_0.get_pk()).await?;

        // THEN the length of the repo is 0
        assert_eq!(repo.data.read().unwrap().len(), 0);
        // AND the item is not returned
        assert_eq!(repo.get(&ITEM1_0.get_pk()).await?, None);

        Ok(())
    }


    #[tokio::test]
    async fn test_get() -> Result<(), Error> {
        // GIVEN a repo with an item
        let repo: InMemory<Item1, i32> = InMemory::new();
        {
            let mut data = repo.data.write().unwrap();
            data.insert(ITEM1_0.get_pk().clone(), ITEM1_0.clone());
        }

        // WHEN getting the product
        let item = repo.get(&ITEM1_0.get_pk()).await?;

        // THEN the product is returned
        assert_eq!(item, Some(ITEM1_0));

        Ok(())
    }

    #[tokio::test]
    async fn test_update() -> Result<(), Error> {
        // GIVEN an empty repo and an item
        let repo: InMemory<Item1, i32> = InMemory::new();

        // WHEN inserting a product
        repo.update(&ITEM1_0).await?;

        // THEN the length of the repo is 1
        assert_eq!(repo.data.read().unwrap().len(), 1);
        // AND the item is returned
        assert_eq!(repo.get(&ITEM1_0.get_pk()).await?, Some(ITEM1_0));

        Ok(())
    }

    #[tokio::test]
    async fn test_create() -> Result<(), Error> {
        // GIVEN an empty repo and an item
        let repo: InMemory<Item1, i32> = InMemory::new();

        // WHEN inserting a product
        repo.create(&ITEM1_0).await?;

        // THEN the length of the repo is 1
        assert_eq!(repo.data.read().unwrap().len(), 1);
        // AND the item is returned
        assert_eq!(repo.get(&ITEM1_0.get_pk()).await?, Some(ITEM1_0));

        Ok(())
    }

}