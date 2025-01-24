
# Payment transaction

## Description

This scenario walks through the process of obtaining payment authorization for a customer

## Objectives / Success Criteria

- **Authorize payments**: (Simplified) A customer should be able to get payment authorization from its bank by contacting it through a network using ISO8583 protocol.


## Milestones / Steps

- [X] **Step 1**: Implement a generic ISO8583 server that handles 11XX and 14XX ISO9593 messsages.
- [ ] **Step 2**: Implement agents check logic at each step (network checks, bank checks) and manage the time-out scenarios
- [ ] **Step 3**: Implement flow tests in a local environment (Docker compose)
- [ ] **Step 4**: Implement flow tests in a remote environment (AWS Fargate)
- [ ] **Step 5**: Implement Github Action workflow for container CD

## Setup Instructions
- **Environment Requirements**: ...
- **Pre-requisites**: ...
- **How to Reproduce**:
    1. ...
    2. ...

## Test Cases
1. **Test Case 1: [Title]**
    - **Input**: 
    - **Expected Output**: 

2. **Test Case 2: [Title]**
    - **Input**: 
    - **Expected Output**: 


## Post-Conditions / Cleanup
- 
