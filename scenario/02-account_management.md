# Customer account management

## Description

This scenario walks through the process where a new customer creates an account and manages it. It tests that the account creation and balance check routes work as expected.


## Objectives

- **Account creation**: A customer can create an account in any of the banks of the ecosystem.
- **Check balance**: A customer can check their current balance.

## Milestones / Steps
- [X] **Step 1**: Define `Customer` model and domain logic
- [X] **Step 2**: Implement tooling for querying custom models in SQL
- [X] **Step 3**: Provision an Aurora DB Cluster and ecosystem configuration in an S3 bucket
- [X] **Step 4**: Implement and deploy lambda http handlers behind an API Gateway connecting to the DB
- [X] **Step 5**: Implement flow tests

## Setup Instructions
- **Environment**: AWS
- **Pre-requisites**: `cargo lambda` and `sam` installed and configured with your AWS credentials.
- **How to Reproduce**:
    1. Run `make setup` to create the database and the S3 bucket
    2. Run `make account_management` to build the lambda functions and deploy a banking agent. 
    3. Run flow tests in `agents/bank/apigateway.rs` with :
        ```
        API_URL=$$(aws cloudformation describe-stacks --stack-name bank-1 \
		--query 'Stacks[0].Outputs[?OutputKey==`BankingAgentApiUrl`].OutputValue' \
		--output text) cargo test --package bank --test apigateway -- test_account_management_flow --exact --show-output --ignored
        ````

## Test Cases
### TODO Fix settings bug
1. [X] **Test Case 1: [Account creation]**
    - **Input**: Randomly generated customer detail and an API endpoint
    - **Step 1**: Create a customer account with `POST /create-account`
    - **Expected Output**: Code: 201

2. [X] **Test Case 2: [Get balance]**
    - **Input**: A user uuid and an API endpoint
    - **Step 1**: `GET /get-balance` 
    - **Expected Output**: Code: 200 and correct user account balance.


## Post-Conditions / Cleanup
- [ ] Delete the created user.
