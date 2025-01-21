//! Utils
//! Tracing initialization
//! RdsClient initialization

// use tracing::instrument;
// use crate::rds_repository::RdsRepository;
// use crate::settings::{get_settings, init_environment};

/// Setup tracing
pub fn setup_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("failed to set tracing subscriber");

}

// Setup repository
// #[instrument]
// pub async fn get_bank_repository() -> RdsRepository {
//     // Get AWS Config
//     let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

//     // Load settings 
//     let environment = init_environment().expect("Failed to initialize environment");
//     let settings = get_settings(&environment).expect("Failed to load configuration");

//     // Initialize Rds Repository
//     RdsRepository::new(&settings.rds, &sdk_config)
// }
