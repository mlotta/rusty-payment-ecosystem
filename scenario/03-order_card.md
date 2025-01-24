# Card lifecycle

## Description
This scenario walks through the process of managing interaction between several agents to create a card lifecycle. It adds a card provider (network) that will take care of create and managing customer cards.

## Objectives 
- **Account and card management**: A network implements customer and card management.
- **Customer notification**: Notify customers of card creation

## Milestones / Steps
- [X] **Step 1**: Implement domain logic including PAN logic
- [ ] **Step 2**: Implement lambda http handlers
- [ ] **Step 3**: Deploy a cluster of an bank and a network that can communicate with each other
- [ ] **Step 4**: Test flow

## Setup Instructions
- **Environment Requirements**: An AWS account
- **Pre-requisites**: 02-account_management
- **How to Reproduce**:
    1. ...
    2. ...

## Test Cases

1. [ ] **Test Case 1: [Account creation]**
    - **Input**: Randomly generated customer detail and an API endpoint
    - **Step 1**: Create a customer account with `POST /create-account`
    - **Expected Output**: Code: 201

2. [ ] **Test Case 2: [Create card contract]**
    - **Input**: A customer with a network account
    - **Step 1**: `POST /create-card-contract` 
    - **Expected Output**: Code: 201

3. [ ] **Test Case 3: [Order card]**
    - **Input**: A customer with a bank account
    - **Step 1**: `POST /order-card` 
    - **Expected Output**: Code: 201 (and notification)

## Post-Conditions / Cleanup
...
