# Remote database connection

## Description

This scenario walks through the process of managing custom models in a remote database.


## Objectives

- **Connect to a remote AWS Aurora DB**
- **Seemlessly query the database**: for common operations such a create, delete, get, list and update. 

## Milestones / Steps
- [X] **Step 1**: Instanciate a remote DB
- [X] **Step 2**: Develop the settings tooling allowing a connection
- [X] **Step 3**: Implement SQL macros to perform any query given a custome model
- [X] **Step 4**: Implement tests

## Setup Instructions
- **Environment**: Manually set up an Aurora DB instance and enable HTTP.
- **How to Reproduce**:
    1. Rename `config/example-base.yaml`to `config/base.yaml` and fill your database credentials.
    2. Run the test in `agents/shared/usecase/rds`

## Test Cases

TODO: describe the tests in the md file
