//! Settings loading

use std::collections::HashMap;

use secrecy::Secret;
use serde::Deserialize;

/// Top level settings
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub log_level: String,
    pub rds: RdsSettings,
    pub agents: AgentSettings,
}

/// Settings for the Amazon Relational Database Service (Amazon RDS) client, primarily the Database & Cluster to access.
#[derive(Debug, Deserialize)]
pub struct RdsSettings {
    pub secret_arn: Secret<String>,
    pub cluster_arn: String,
    pub db_instance: String,
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
}

const DEFAULT_ENVIRONMENT: &str = "local";
const DEFAULT_LOG_LEVEL: &str = "trace";
const DEFAULT_BACKTRACE: &str = "1";

/// Attempt to find the environment, and preset any environment variables.
/// Valid environments are in [Environment].
pub fn init_environment() -> Result<Environment, SettingsError> {
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| DEFAULT_ENVIRONMENT.into())
        .try_into()
        .expect("failed to parse APP_ENVIRONMENT");

    // Convenient for development. Production should set its flags itself.
    if !matches!(environment, Environment::Production) {
        if std::env::var("RUST_LIB_BACKTRACE").is_err() {
            std::env::set_var("RUST_LIB_BACKTRACE", DEFAULT_BACKTRACE)
        }
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", DEFAULT_LOG_LEVEL)
        }
    }

    Ok(environment)
}

/// Load settings, given an [Environment].
/// Looks in base.yaml, {environment}.yaml, and then APP_* env variables.
pub fn get_settings(environment: &Environment) -> Result<Settings, SettingsError> {
    let base_dir = std::env::current_dir().expect("Failed to determine cwd");
    let config_dir = base_dir.join("../../config");
    let base_yaml = "base.yaml";
    let environment_yaml = format!("{}.yaml", environment.as_str());
    let ecosystem_yaml = "ecosystem.yaml";

    let settings_loader = config::Config::builder()
        .add_source(config::File::from(config_dir.join(base_yaml)))
        .add_source(config::File::from(config_dir.join(environment_yaml)).required(false))
        .add_source(config::File::from(config_dir.join(ecosystem_yaml)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"), // By convention, to deconflict variables with _s.
        )
        .build()
        .map_err(SettingsError::Config)?;

    settings_loader
        .try_deserialize::<Settings>()
        .map_err(SettingsError::Config)
}

/// Valid environments for this app.
pub enum Environment {
    Local,
    Test,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Test => "test",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "test" => Ok(Environment::Test),
            "production" => Ok(Environment::Production),
            other => Err(format!(
                "Unknown environment {other}. Please use 'local', 'test', or 'production'."
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init_local_environment() -> Result<(), SettingsError> {
        let envi = init_environment().expect("failed to load event");

        assert_eq!(envi.as_str(), Environment::Local.as_str());
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_get_local_settings() -> Result<(), SettingsError> {
        let envi = init_environment().expect("failed to load event");
        let settings = get_settings(&envi).expect("failed to load settings");

        assert_eq!(settings.log_level, "info");
        assert_eq!(settings.rds.db_instance, "bank_1");

        dbg!(settings.agents.bank);
        dbg!(settings.agents.network);
        assert_eq!("", "1");
        Ok(())
    }
}
