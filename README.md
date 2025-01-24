# rusty-payment-ecosystem

This project is a Rust, PostgreSQL and AWS implementation of a banking ecosystem in which bank customers can order debit cards and perform payments through a network. The ecosystem is greatly simplified and serves only pedagological purposes.

**Status**: Work in Progress. The following serves both as a description of the current state of the project and as a roadmap for future development.

## Description of the ecosystem

### Customer account management

- A customer can create an account in any of the banks of the ecosystem with `POST /create-account`.
- A customer can check their balance with a call to `GET /get-balance`.

### Card lifecycle

- A customer with a bank account can order a card with `POST /order-card`.
  - The bank processes the order and transfers it to one of the networks for which it is has an account. The bank will create an account for the customer with `POST /create-customer-account` (for simplification, we merge the customer, account and card contracts).
  - The bank then creates a card contract with the network `POST /create-card`. 
  - The network starts the production of the card and notifies the customer when it is ready (AWS SNS).
  - Upon reception of the card, the customer can activate it with their first payment.

### Payment transactions

- A cardholder can start transactions using the ISO 8583 communication protocol. For simplification, we limit the number of interactions to:
  - The cardholder connects to a network to start an authorization request (MTI: 1100)
  - The network performs checks (e.g. expiration date, ...) and connects to the card's associated bank (MTI: 1100)
  - The bank performs checks (e.g. balance, fraud,...) and replies with an authorization response (MTI: 1110)
  - The network transfers the answer to the cardholder (MTI: 1110)
- If the bank doesn't answer before a pre-set time, the network issues a 1420 reversal advice message to the bank to cancel the original 1110. The cardholder is notified of a time-out error (MTI 1110 with code 91 in field DE-39).
- If the network receives confirmation from the bank but times out on informing the cardholder, the cardholder will issue a 1420 reversal message to the bank through the network. The network doesn't propagate the message if the original transaction was refused.
- For simplification, a successful transaction will update the balance (skipping clearing and settlement).

[comment]: <> (- Create an RDS Aurora Serverless Postgresql with API endpoint activated. Create a secret in Secret Manager with the credentials of the database and store them in base.yaml)

## Implementation

We describe three types of agents: Banks, Networks, and Cardholders.  The crates `bank`, `network` and `cardholder` respectively implement the code executed by each of these agents. They all rely on the `shared` crate which uses an hexagonal architecture pattern to provide off-the-shelf interface implementations :
- [X] Repository : AWS RDS (including a macro to generate sql code from a struct's definition), in memory
- [ ] Recipient : AWS SNS
- [ ] Lambda HTTP events
- [ ] An ISO 8583 server (based on [iso8583_rs](https://github.com/rkbalgi/iso8583_rs/tree/master?tab=readme-ov-file))

Each agent is deployed independantly on AWS in a VPC described by:
- An Aurora Serveless PostgreSQL database
- An API Gateway mapping to :
- Lambdas for the treatment of the HTTP requests
- A Fargate instance processing the ISO 8583 requests

The configuration of these agents is described in the `/config/ecosystem.yaml` file.

## Methodology and general guidance

This project uses AWS features for the deployment of the agents. Even though most of the core features can be tested locally, keep in mind that the complete configuration of the project will result in AWS costs.


[comment]: <> (Rename and fill `config/base.yaml and require deployment of aurora rds`)
[comment]: <> (Credits: https://github.com/aws-samples/serverless-rust-demo/tree/main, https://github.com/awslabs/aws-sdk-rust/tree/main/examples/cross_service/rest_ses)
