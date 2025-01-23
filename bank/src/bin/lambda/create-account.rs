use shared::utils::setup_tracing;
use bank::utils::get_bank_repository;
use lambda_http::{service_fn, Request};

type E = Box<dyn std::error::Error + Send + Sync + 'static>;


#[tokio::main]
async fn main() -> Result<(), E> {
    // Initialize logger
    setup_tracing();

    // Initialize repository
    let repo = get_bank_repository().await;

    lambda_http::run(service_fn(|event: Request| bank::apigateway::create_account(&repo, event))).await?;
    Ok(())
}
