//! Implementation of a Repository for a Rds Client
use std::marker::PhantomData;
use std::sync::Arc;

use crate::{error::InterfaceError, QuerySet, Val};
use crate::{
    ports::secondary::{Create, Delete, Get, List, Repository, Update},
    rds_client::RdsClient,
};
use async_trait::async_trait;
use aws_sdk_rdsdata::types::SqlParameter;
use aws_sdk_rdsdata::{
    error::SdkError,
    operation::execute_statement::{ExecuteStatementError, ExecuteStatementOutput},
    types::RecordsFormatType,
};
use uuid::Uuid;

/// Build a `Vec<SqlParameter>` to use in ExecuteStatementBuilder::set_parameters.
/// from an items fields
pub trait GetFieldsAsParams {
    fn get_fields_as_params(&self) -> Option<Vec<SqlParameter>>;
}

pub struct RdsRepository<T, Q>
where
    T: Val + GetFieldsAsParams,
    Q: QuerySet<T> + std::marker::Sync,
{
    client: Arc<RdsClient>,
    queryset: Box<Q>,

    _marker_val: PhantomData<T>,
}

impl<T, Q> RdsRepository<T, Q>
where
    T: Val + GetFieldsAsParams,
    Q: QuerySet<T> + std::marker::Sync,
{
    /// Create a table with name {table} in the remote database
    pub fn new(client: Arc<RdsClient>, queryset: Box<Q>) -> Self {
        RdsRepository {
            client,
            queryset,
            _marker_val: PhantomData,
        }
    }

    /// Create the remote table
    pub async fn create_table(&self) -> Result<(), InterfaceError> {
        self.client
            .execute_statement()
            .sql(self.queryset.create_table())
            .send()
            .await
            .map_err(|err| InterfaceError::RdsError(err.into()))?;
        Ok(())
    }

    /// !!! DROP THE REMOTE TABLE !!!
    pub async fn drop_table(&self) -> Result<(), InterfaceError> {
        self.client
            .execute_statement()
            .sql(self.queryset.drop_table())
            .send()
            .await
            .map_err(|err| InterfaceError::RdsError(err.into()))?;
        Ok(())
    }

    #[allow(clippy::result_large_err)]
    fn parse_rds_output(
        &self,
        statement: Result<ExecuteStatementOutput, SdkError<ExecuteStatementError>>,
    ) -> Result<Vec<T>, InterfaceError>
    where
        T: serde::de::DeserializeOwned,
    {
        // Did the request succeed?
        let data = match statement {
            Ok(data) => Ok(data),
            Err(err) => Err(InterfaceError::RdsError(err.into())),
        }?;

        // Are there records?
        let records = match data.formatted_records() {
            Some(records) => Ok(records),
            None => Err(InterfaceError::Other(
                "Amazon RDS Data did not include records in their response.".to_string(),
            )),
        }?;

        // Can we parse the records?
        match serde_json::from_str::<Vec<T>>(records.to_string().as_str()) {
            Ok(items) => Ok(items),
            Err(e) => Err(InterfaceError::FromFields(format!(
                "Failed to parse formatted records: {e}"
            ))),
        }
    }
}

#[async_trait]
impl<T, Q> Create<T> for RdsRepository<T, Q>
where
    T: Val + GetFieldsAsParams,
    Q: QuerySet<T> + std::marker::Sync,
{
    async fn create(&self, item: &T) -> Result<(), InterfaceError> {
        self.client
            .execute_statement()
            .sql(self.queryset.create())
            .set_parameters(item.get_fields_as_params())
            .send()
            .await
            .map_err(|err| InterfaceError::RdsError(err.into()))?;
        Ok(())
    }
}

#[async_trait]
impl<T, Q> Get<T> for RdsRepository<T, Q>
where
    T: Val + GetFieldsAsParams,
    Q: QuerySet<T> + std::marker::Sync,
{
    async fn get(&self, id: &Uuid) -> Result<Option<T>, InterfaceError> {
        let statement = self
            .client
            .execute_statement()
            .sql(self.queryset.get("uuid"))
            .set_parameters(Some(vec![aws_sdk_rdsdata::types::SqlParameter::builder()
                .name("uuid".to_string())
                .value(aws_sdk_rdsdata::types::Field::StringValue(
                    id.to_string().clone(),
                ))
                .type_hint(aws_sdk_rdsdata::types::TypeHint::Uuid)
                .build()]))
            .format_records_as(RecordsFormatType::Json)
            .send()
            .await;

        let items: Vec<T> = self.parse_rds_output(statement)?;

        if items.len() > 1 {
            // There should only be one record
            return Err(InterfaceError::Other(format!(
                "Received multiple results for id: {:}",
                id
            )));
        }

        if items.is_empty() {
            return Ok(None);
        }

        let item = match items.first() {
            Some(item) => Ok(item),
            None => Err(InterfaceError::Other(
                "Somehow len() == 1 but get(0) is None".to_string(),
            )),
        }?;
        Ok(Some(item.to_owned()))
    }
}

#[async_trait]
impl<T, Q> Delete<T> for RdsRepository<T, Q>
where
    T: Val + GetFieldsAsParams,
    Q: QuerySet<T> + std::marker::Sync,
{
    async fn delete(&self, id: &Uuid) -> Result<(), InterfaceError> {
        self.client
            .execute_statement()
            .sql(self.queryset.delete("uuid"))
            .set_parameters(Some(vec![aws_sdk_rdsdata::types::SqlParameter::builder()
                .name("uuid".to_string())
                .value(aws_sdk_rdsdata::types::Field::StringValue(
                    id.to_string().clone(),
                ))
                .type_hint(aws_sdk_rdsdata::types::TypeHint::Uuid)
                .build()]))
            .send()
            .await
            .map(|_| Ok(()))
            .map_err(|err| InterfaceError::RdsError(err.into()))?
    }
}

#[async_trait]
impl<T, Q> Update<T> for RdsRepository<T, Q>
where
    T: Val + GetFieldsAsParams,
    Q: QuerySet<T> + std::marker::Sync,
{
    async fn update(&self, item: &T) -> Result<(), InterfaceError> {
        self.client
            .execute_statement()
            .sql(self.queryset.update())
            .set_parameters(item.get_fields_as_params())
            .send()
            .await
            .map_err(|err| InterfaceError::RdsError(err.into()))?;
        Ok(())
    }
}

#[async_trait]
impl<T, Q> List<T> for RdsRepository<T, Q>
where
    T: Val + GetFieldsAsParams,
    Q: QuerySet<T> + std::marker::Sync,
{
    async fn list(&self) -> Result<Vec<T>, InterfaceError> {
        let statement = self
            .client
            .execute_statement()
            .sql(self.queryset.list())
            .format_records_as(RecordsFormatType::Json)
            .send()
            .await;

        self.parse_rds_output(statement)
    }
}

#[async_trait]
impl<T, Q> Repository<T> for RdsRepository<T, Q>
where
    T: Val + GetFieldsAsParams,
    Q: QuerySet<T> + std::marker::Sync,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{get_settings, init_environment};
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};
    use sql_macros::struct_to_sql;
    use uuid::Uuid;

    // Define structures
    #[derive(Deserialize, Serialize, PartialEq)]
    #[struct_to_sql]
    struct Item1 {
        uuid: Uuid,
        field1: i32,
    }

    // Gen item
    fn gen_item() -> Item1 {
        Item1 {
            uuid: Uuid::new_v4(),
            field1: 3,
        }
    }

    async fn get_item1_repository() -> RdsRepository<Item1, Item1QuerySet<Item1>> {
        // Get AWS Config
        let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

        // Load settings
        let environment = init_environment().expect("Failed to initialize environment");
        let settings = get_settings(&environment).expect("Failed to load configuration");

        // Get client
        let client = Arc::new(RdsClient::new(&settings.rds, &sdk_config));

        // Initialize the Item1 queryset
        let queryset: Box<Item1QuerySet<Item1>> = Box::new(Item1::queryset());

        // Initialize Rds Customer Repository
        let repo: RdsRepository<Item1, Item1QuerySet<Item1>> = RdsRepository::new(client, queryset);
        repo
    }

    #[tokio::test]
    #[ignore]
    async fn test_queryset() -> Result<(), InterfaceError> {
        let repo: RdsRepository<Item1, Item1QuerySet<Item1>> = get_item1_repository().await;

        assert_eq!(repo.queryset.table(), "Item1".to_string());
        assert_eq!(
            repo.queryset.drop_table(),
            "DROP TABLE IF EXISTS Item1".to_string()
        );
        assert_eq!(
            repo.queryset.create_table(),
            "CREATE TABLE IF NOT EXISTS Item1 (uuid UUID, field1 INTEGER)".to_string()
        );
        Ok(())
    }

    /// Accessing the remote database, may result in AWS service fees
    #[tokio::test]
    #[serial_test::serial]
    #[ignore]
    async fn test_create_and_delete_db() -> Result<(), InterfaceError> {
        // GIVEN an empty repository
        let repo: RdsRepository<Item1, Item1QuerySet<Item1>> = get_item1_repository().await;

        // WHEN we create and delete a table
        // THEN we get no error
        let _ = repo.create_table().await?;
        let _ = repo.drop_table().await?;
        Ok(())
    }

    //// Accessing the remote database, may result in AWS service fees
    #[tokio::test]
    #[serial_test::serial]
    #[ignore]
    async fn test_all_empty() -> Result<(), InterfaceError> {
        // GIVEN a repository with an empty table
        let repo: RdsRepository<Item1, Item1QuerySet<Item1>> = get_item1_repository().await;
        let _ = repo.create_table().await?;

        // WHEN we list all items
        let all = repo.list().await?;

        // THEN we get an empty list
        assert_eq!(all.len(), 0);

        let _ = repo.drop_table().await?;
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    #[ignore]
    async fn test_create_entry() -> Result<(), InterfaceError> {
        // GIVEN a repository with an empty table and an item
        let repo: RdsRepository<Item1, Item1QuerySet<Item1>> = get_item1_repository().await;
        let _ = repo.drop_table().await?;
        let _ = repo.create_table().await?;
        let item = gen_item();

        // WHEN we create an entry
        let _ = repo.create(&item).await?;

        // THEN we get no errors and there is one item in the table
        let all = repo.list().await?;
        let _ = repo.drop_table().await?;
        assert_eq!(all.len(), 1);
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    #[ignore]
    async fn test_delete_entry() -> Result<(), InterfaceError> {
        // GIVEN a repository with an empty table and an item
        let repo: RdsRepository<Item1, Item1QuerySet<Item1>> = get_item1_repository().await;
        let _ = repo.drop_table().await?;
        let _ = repo.create_table().await?;
        let item = gen_item();

        // WHEN we create and delete an entry
        let _ = repo.create(&item).await?;
        let _ = repo.delete(&item.uuid).await?;

        // THEN we get no errors and there is no item in the table
        let all = repo.list().await?;
        let _ = repo.drop_table().await?;
        assert_eq!(all.len(), 0);
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    #[ignore]
    async fn test_update_entry() -> Result<(), InterfaceError> {
        // GIVEN a repository with an empty table and an item
        let repo: RdsRepository<Item1, Item1QuerySet<Item1>> = get_item1_repository().await;
        let _ = repo.drop_table().await?;
        let _ = repo.create_table().await?;
        let mut item = gen_item();

        // WHEN we create and update an entry
        let _ = repo.create(&item).await?;
        item.field1 += 1;
        let _ = repo.update(&item).await?;

        // THEN we get no errors, there is one entry in the table
        // and the modifications where applied
        let resp_item: Item1 = repo.get(&item.uuid).await?.unwrap();
        assert_eq!(item.field1, resp_item.field1);
        let all = repo.list().await?;
        let _ = repo.drop_table().await?;
        assert_eq!(all.len(), 1);
        Ok(())
    }
}
