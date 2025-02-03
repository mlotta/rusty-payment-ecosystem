use shared::error::InterfaceError;
use shared::rds_client::RdsClient;
use shared::settings::get_settings;

/// Initiate an agent's custom database
/// TODO: Create and manage an User for each agent, currently sharing the admin User
async fn init_agent(client: &RdsClient, name: &str) -> Result<(), InterfaceError> {
    client
        .execute_statement()
        .sql(format!("CREATE DATABASE {}", name))
        .send()
        .await
        .map_err(|err| InterfaceError::RdsError(err.into()))?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), InterfaceError> {
    // Get AWS Config
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    // Load settings
    let settings = get_settings().await.expect("Failed to load configuration");

    // Get client
    let client = RdsClient::new(&settings.rds, &sdk_config);

    // Create a single agent
    init_agent(&client, "bank_1").await
}
