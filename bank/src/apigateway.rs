use lambda_http::{http::{Method, StatusCode}, IntoResponse, Request, RequestExt, RequestPayloadExt, Response};
use tracing::{error, info, instrument, warn};
use serde_json::json;
use crate::usecase::BankRepository;

type E = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Get the balance of a given customer
#[instrument(skip(repo))]
pub async fn get_balance(repo: &dyn BankRepository, event: Request) -> Result<impl IntoResponse, E>{
    // Ensure GET method
    if event.method() != Method::GET {
        return Ok(response(
            StatusCode::METHOD_NOT_ALLOWED, 
            json!({"message": "Method Not Allowed"}).to_string()))
    }
    
    // Retrieve customer ID from event
    //
    // If the event doesn't contain a customer UUID, we return a 400 Bad Request.
    let path_parameters = event.path_parameters();
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
    // TODO: Don't return the full customer object
    info!("Fetching balance for user uuid {}", uuid);
    let customer = crate::domain::get_balance(repo, uuid).await;

    // Return response
    Ok(match customer {
        // Found
        Ok(Some(customer)) => response(StatusCode::OK, json!(customer).to_string()),
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


/// Create a customer account
#[instrument(skip(repo))]
pub async fn create_account(repo: &dyn BankRepository, event: Request) -> Result<impl IntoResponse, E>{
    // Ensure POST method
    if event.method() != Method::POST {
        return Ok(response(
            StatusCode::METHOD_NOT_ALLOWED, 
            json!({"message": "Method Not Allowed"}).to_string()))
    }
    // Read customer from request
    let customer: crate::models::customer::Customer = match event.payload() {
        Ok(Some(customer)) => customer,
        Ok(None) => {
            warn!("Missing account details in request body");
            return Ok(response(
                StatusCode::BAD_REQUEST, 
            json!({"message": "Missing account details in request body"}).to_string()))
        },
        Err(err) => {
            warn!("Failed to parse account details from request body: {}", err);
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({"message": "Failed to parse account detials from request body"}).to_string(),
            ));
        }
    };
    info!("Parsed customer: {:?}", customer);

    // Create customer
    let resp = crate::domain::create_account(repo, &customer).await;
    dbg!(&customer.uuid);

    // Return response
    Ok(match resp {
        // Found
        Ok(_) => {
            info!("Created customer {:?}", customer.name);
            response(
                StatusCode::CREATED,
                json!({"message": "Account created"}).to_string()
            )
        } 

        // Error
        Err(err) => {
            error!("Failed to crate an account {}: {}", customer.name, err);
            response(
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"message": "Failed to create account"}).to_string(),
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

