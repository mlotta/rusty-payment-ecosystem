# payment-ecosystem

This project is a Rust, PostgreSQL and AWS implementation of a banking ecosystem in which bank customers can order debit cards and perform payments. It serves only pedagological purposes.

For the sake of simplicity, we don't represent the entire banking ecosystem, but only cardholders, banks and networks.

## Card lifecycle

- 

## Payment transactions

[comment]: <> (- Create an RDS Aurora Serverless Postgresql with API endpoint activated. Create a secret in Secret Manager with the credentials of the database and store them in base.yaml)


## Also implementing

The `shared` library uses an hexagonal architecture pattern that provides off-the-shelf implementations of database, http requests and lambda event interfaces for custom models.

[comment]: <> (Rename and fill `config/base.yaml`)
[comment]: <> (Detail the implemented repository, handlers, etc)
[comment]: <> (Credits: https://github.com/aws-samples/serverless-rust-demo/tree/main, https://github.com/awslabs/aws-sdk-rust/tree/main/examples/cross_service/rest_ses)