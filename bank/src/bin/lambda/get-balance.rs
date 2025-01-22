use shared::utils::setup_tracing;
use bank::{usecase::BankRepository, utils::get_bank_repository};
use lambda_http::{service_fn, Request};

type E = Box<dyn std::error::Error + Send + Sync + 'static>;

fn populate(repo: &dyn BankRepository){
// repo.create()

}

#[tokio::main]
async fn main() -> Result<(), E> {
    // Initialize logger
    setup_tracing();

    // Initialize repository
    let repo = get_bank_repository().await;

    // Populate the repository when testing
    populate(&repo);

    lambda_http::run(service_fn(|event: Request| bank::apigateway::get_balance(&repo, event))).await?;
    Ok(())
}
