pub mod usecase;
pub mod ports;
// pub mod domain;
pub mod error;

pub mod utils;
pub mod settings;
pub mod rds_client;


// Define requirement for Val
pub trait Val: Default + Send + Sync + Clone {}
impl<T> Val for T where T: Default + Send + Sync + Clone {}


/// Queryset for SQL implementations
pub trait QuerySet<T> {
    /// Table name
    fn table(&self) -> String;

    /// SQL query to create a new table
    fn create_table(&self) -> String;

    /// SQL query to drop a table
    fn drop_table(&self) -> String;

    /// SQL query to delete an object by field (prepared)
    fn delete(&self, field_name: &str) -> String;

    /// SQL query to get an object by field (prepared)
    fn get(&self, field_name: &str) -> String;

    /// SQL query to create an object (prepared)
    fn create(&self) -> String;

    /// SQL query to update an object (prepared)
    fn update(&self) -> String;

    /// SQL query to list all items
    fn list(&self) -> String;
}
            