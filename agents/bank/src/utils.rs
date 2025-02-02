#[allow(unused_imports)]
use shared::settings::get_settings;
use tracing::instrument;

// Setup repository
#[instrument]
#[cfg(test)]
pub async fn get_bank_repository() -> impl crate::usecase::BankRepository {
    crate::usecase::memory::BankMemoryRepository::new()
}

// Setup repository
#[instrument]
#[cfg(not(test))]
pub async fn get_bank_repository() -> impl crate::usecase::BankRepository {
    // Get AWS Config
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    // Load settings
    let settings = get_settings().await.expect("Failed to load configuration");

    // Initialize Rds Repository
    crate::usecase::rds::BankRdsRepository::new(&settings.rds, &sdk_config)
}
