//! AWS Testing
//!

use bank::models::customer::Customer;
use pretty_assertions::assert_eq;
use rand::Rng;
use reqwest::StatusCode;
use uuid::Uuid;

type E = Box<dyn std::error::Error + Send + Sync + 'static>;

pub fn get_random_customer() -> Customer {
    let mut rng = rand::thread_rng();

    // let account_number: String = (0..11)
    //     .map(|_| rng.gen_range(0..10).to_string())
    //     .collect();
    Customer {
        uuid: Uuid::new_v4(),
        name: format!("customer-{}", rng.gen_range(1..=1000)),
        // account_number: account_number,
        balance: rng.gen_range(0..=1000),
    }
}

#[tokio::test]
async fn test_flow() -> Result<(), E> {
    let client = reqwest::Client::new();
    let api_url: String = "http://localhost:9000/lambda-url".to_string();

    let customer = get_random_customer();
    dbg!(&customer.uuid);

    // Create account for customer
    println!(
        "Creating an account for customer with name {}",
        customer.name
    );
    let res = client
        .post(format!("{}/create-account", api_url))
        .json(&customer)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::CREATED);

    // Get balance
    println!("Get customer balance");
    let res = client
        .get(format!("{}/get-balance/uuid/{}", api_url, customer.uuid))
        .send()
        .await?;
    dbg!(&res);
    assert_eq!(res.status(), StatusCode::OK);
    let res_customer: Customer = res.json().await?;
    assert_eq!(customer.balance, res_customer.balance);

    Ok(())
}
