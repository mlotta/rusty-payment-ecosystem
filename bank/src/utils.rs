
use tracing::instrument;
use shared::settings::{get_settings, init_environment};


// Setup repository
#[instrument]
pub async fn get_bank_repository() -> impl crate::usecase::BankRepository {
    // Get AWS Config
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    // Load settings
    let environment = init_environment().expect("Failed to initialize environment");
    let settings = get_settings(&environment).expect("Failed to load configuration");

    // Initialize Rds Repository
    crate::usecase::rds::BankRdsRepository::new(&settings.rds, &sdk_config)
}