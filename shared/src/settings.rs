//! Settings loading

use std::collections::HashMap;
use secrecy::Secret;
use serde::Deserialize;
use config::{builder::DefaultState, ConfigBuilder};

/// Top level settings
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub rds: RdsSettings,
    pub agents: AgentSettings,
}


/// Settings for the Amazon Relational Database Service (Amazon RDS) client, primarily the Database & Cluster to access.
#[derive(Debug, Deserialize)]
pub struct RdsSettings {
    pub secretarn: Secret<String>,
    pub clusterarn: String,
    pub dbinstance: String,
}

/// Settings for a given agent, i.e. a cardholder, a bank, a network, ...
#[derive(Debug, Deserialize)]
pub struct AgentSettings {
    // Cardholder(CardholderSettings),
    pub bank: HashMap<String, BankSettings>,
    pub network: HashMap<String, NetworkSettings>,
}

#[derive(Debug, Deserialize)]
pub struct BankSettings {
    pub issuer_identification_numbers: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct NetworkSettings {
    pub major_industry_identifier: u8,
    pub issuer_identification_numbers: HashMap<String, String>,
}

/// Any errors when loading the settings. Why are environments so complicated.
#[derive(Debug)]
pub enum SettingsError {
    Config(config::ConfigError),
    #[cfg(not(test))]
    S3(String)
}


/// Get ecosystem S3 bucket
#[cfg(not(test))]
async fn get_ecosystem_settings(settings_loader: ConfigBuilder<DefaultState>) -> Result<ConfigBuilder<DefaultState>, SettingsError> {
    // Get AWS Config
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    let bucket_name = std::env::var("CONFIG_FILE_BUCKET").expect("CONFIG_FILE_BUCKET must be set");
    let key = std::env::var("CONFIG_FILE_KEY").expect("CONFIG_FILE_KEY must be set");
    let client = aws_sdk_s3::Client::new(&sdk_config);

    let response = client
        .get_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await
        .map_err(|err| SettingsError::S3(err.to_string()))?;

    let data = response.body.collect().await.map_err(|err| SettingsError::S3(err.to_string()))?;
    let ecosystem_yaml = String::from_utf8(data.to_vec()).map_err(|err| SettingsError::S3(err.to_string()))?;

    Ok(settings_loader.add_source(config::File::from_str(&ecosystem_yaml, config::FileFormat::Yaml)))

}

/// Get rds settings from env
#[cfg(not(test))]
async fn get_db_settings(settings_loader: ConfigBuilder<DefaultState>) -> Result<ConfigBuilder<DefaultState>, SettingsError>{
    // Add environment variables as a source
    Ok(settings_loader.add_source(
        config::Environment::with_prefix("DB") // Use "DB" as the prefix for database-related environment variables
            .separator("_") 
    ))
}

/// Get ecosystem config from local file
#[cfg(test)]
async fn get_ecosystem_settings(settings_loader: ConfigBuilder<DefaultState>) -> Result<ConfigBuilder<DefaultState>, SettingsError> {
    let base_dir = std::env::current_dir().expect("Failed to determine cwd");
    let config_dir = base_dir.join("../config");
    let ecosystem_yaml = "ecosystem-config.yaml";

    Ok(settings_loader.add_source(config::File::from(config_dir.join(ecosystem_yaml))))
}

/// Get rds settings from local file
#[cfg(test)]
async fn get_db_settings(settings_loader: ConfigBuilder<DefaultState>) -> Result<ConfigBuilder<DefaultState>, SettingsError>{
    let base_dir = std::env::current_dir().expect("Failed to determine cwd");
    let config_dir = base_dir.join("../config");
    let database_yaml = "local.yaml";

    Ok(settings_loader.add_source(config::File::from(config_dir.join(database_yaml))))

}

/// Load settings
/// Looks for database settings in the environment and the ecosystem config
/// in an S3 bucket.
pub async fn get_settings() -> Result<Settings, SettingsError> {
    let mut settings_loader = config::Config::builder();

    settings_loader = get_ecosystem_settings(settings_loader).await?;
    settings_loader = get_db_settings(settings_loader).await?;

    let settings_loader: config::Config = settings_loader
        .build()
        .map_err(SettingsError::Config)?;

    settings_loader
        .try_deserialize::<Settings>()
        .map_err(SettingsError::Config)
    
}


#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_get_local_settings() -> Result<(), SettingsError> {
        let settings = get_settings().await.expect("failed to load settings");

        // assert_eq!(settings.log_level, "info");
        // assert_eq!(settings.rds.db_instance, "bank_1");

        dbg!(settings.agents.bank);
        dbg!(settings.agents.network);
        assert_eq!("", "1");
        Ok(())
    }
}
