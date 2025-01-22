use lambda_http::{http::StatusCode, IntoResponse, Request, RequestExt, Response};
use tracing::{error, info, instrument, warn};
use serde_json::json;
use crate::usecase::BankRepository;

type E = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Get the balance of a given customer
#[instrument(skip(repo))]
pub async fn get_balance(repo: &dyn BankRepository, event: Request) -> Result<impl IntoResponse, E>{
    // Retrieve customer ID from event
    //
    // If the event doesn't contain a customer UUID, we return a 400 Bad Request.
    let path_parameters = event.path_parameters();
    error!("{:?}", &event);
    error!("{:?}", &path_parameters);
    let uuid = match path_parameters.first("uuid") {
        Some(uuid) => uuid,
        None => {
            warn!("Missing 'uuid' parameter in path");
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({ "message": "Missing 'uuid' parameter in path" }).to_string(),
            ));
        }
    };

    // Validate the UUID format
    let uuid = match uuid::Uuid::parse_str(uuid){
        Ok(parsed_uuid) => parsed_uuid,
        Err(e) => {
            warn!("Invalid UUID format: {}", e);
            return Ok(response(
                StatusCode::BAD_REQUEST, 
                json!({"message": "Invalid UUID format"}).to_string(),
            ));
        }
    };

    // Retrieve the balance
    info!("Fetching balance for user uuid {}", uuid);
    let balance = crate::domain::get_balance(repo, uuid).await;

    // Return response
    Ok(match balance {
        // Found
        Ok(Some(balance)) => response(StatusCode::OK, json!({"balance": balance}).to_string()),
        // Doesn't exist
        Ok(None) => {
            warn!("Customer not found: {}", uuid);
            response(
                StatusCode::NOT_FOUND,
                json!({"message": "Customer not found"}).to_string(),
            )
        }
        // Error
        Err(err) => {
            error!("Error fetching the customer: {}", err);
            response(
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"message": "Error fetching the customer"}).to_string(),
            )
        }
    })
}


/// HTTP Response with a JSON payload
fn response(status_code: StatusCode, body: String) -> Response<String> {
    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
}