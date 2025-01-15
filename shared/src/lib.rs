pub mod usecase;
pub mod ports;
use std::hash::Hash;

// Define requirements for Key
pub trait Key: Default + Eq + Hash + Send + Sync + Clone {}
impl<T> Key for T where T: Default + Eq + Hash + Send + Sync + Clone {}

// Define requirement for Val
pub trait Val: Default + Send + Sync + Clone{}
impl<T> Val for T where T: Default + Send + Sync + Clone {}

/// Define the primary key of of a type
/// K: type of the key
pub trait PrimaryKey<K>{
    fn get_pk(&self) -> K;
}