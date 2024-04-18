# Rust AWS Lambda and Step Functions

## Goals
- Rust AWS Lambda function
- Step Functions workflow coordinating Lambdas
- Orchestrate data processing pipeline

## Rust Lambda Functionality
This project comprises two AWS Lambda functions designed to process textual data for word frequency analysis: `word-frequency` and `frequent-words`. Each function is tailored to perform specific tasks related to word counting and frequency analysis.


## Demo
![Video Demo](./video.mp4)

## Project Steps
### Create Rust Lambda Project
1. Use `cargo lambda new <PROJECT_NAME>` to create lambda project.
2. Add necessary dependencies to `Cargo.toml`.
3. Implement functions in `main.rs`.

### Data Processing Pipeline
1. word-frequency
- Functionality: This function processes raw text input to count the frequency of each word. It uses regular expressions to identify and count words in the text.
- Input: a JSON payload containing a single string of text under the key input.
- Output: a JSON object with a `HashMap` where each key is a word and the value is the frequency of that word in the input text.
2. frequent-words
- Functionality: This function analyzes a given set of word counts and determines the top 10% most frequent words. It uses a min-heap to efficiently filter and return these words.
- Input: a JSON payload with a `HashMap<String, usize>` representing word counts where the string represents the word, and usize is the count of occurrences.
- Output: a JSON array containing only the top 10% frequent words sorted in descending order of frequency.
### Test Locally
1. Use `cargo lambda watch` to test lambda functions locally. To test my function, please follow the commands:
   ```
   cd word-frequency
   cargo lambda watch
   cargo lambda invoke --data-file input.json
   ```
   ```
   cd frequent-words
   cargo lambda watch
   cargo lambda invoke --data-file input.json
   ```
### Deploy on AWS Lambda
1. Create a role with policies `AWSLambda_FullAccess`, `AWSLambdaBasicExecutionRole`, `IAMFullAccess`.
2. Obtain the binary file by building the project:
    ```
    cargo lambda build --release
    ```
3. Make sure to set up the AWS configuration and deploy by:
    ```
    cargo lambda deploy --region <REGION> --iam-role <ROLE_ARN>
    ```
Then you can check your AWS Lambda function on AWS Lambda.

### Build GitLab CI/CD
Create `.gitlab-ci.yml` to build a CI/CD pipeline to build and deploy the Rust Lambda to AWS. Here is the part of my `.gitlab-ci.yml` to build CI/CD pipeline:
```yml
stages:
  - word-frequency-stage
  - frequent-words-stage

word-frequency-stage:
  stage: word-frequency-stage
  image: rust:latest
  script:
    - rustup default stable
    - apt-get update && apt-get install -y wget unzip xz-utils
    - wget https://ziglang.org/download/0.9.1/zig-linux-x86_64-0.9.1.tar.xz
    - tar -xf zig-linux-x86_64-0.9.1.tar.xz -C /usr/local
    - export PATH=$PATH:/usr/local/zig-linux-x86_64-0.9.1
    - cargo install cargo-lambda
    - apt-get install -y zip
    - apt-get install -y musl-tools
    - rustup target add x86_64-unknown-linux-musl
    - cd word-frequency
    - export AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID
    - export AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY
    - export AWS_DEFAULT_REGION=$AWS_DEFAULT_REGION
    - cargo lambda build --release
    - cargo lambda deploy
  only:
    - main

frequent-words-stage:
  stage: frequent-words-stage
  image: rust:latest
  script:
    - rustup default stable
    - apt-get update && apt-get install -y wget unzip xz-utils
    - wget https://ziglang.org/download/0.9.1/zig-linux-x86_64-0.9.1.tar.xz
    - tar -xf zig-linux-x86_64-0.9.1.tar.xz -C /usr/local
    - export PATH=$PATH:/usr/local/zig-linux-x86_64-0.9.1
    - cargo install cargo-lambda
    - apt-get install -y zip
    - apt-get install -y musl-tools
    - rustup target add x86_64-unknown-linux-musl
    - cd frequent-words
    - export AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID
    - export AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY
    - export AWS_DEFAULT_REGION=$AWS_DEFAULT_REGION
    - cargo lambda build --release
    - cargo lambda deploy
  only:
    - main
    

```

### Build Corresponding Step Functions
1. Create a new State Machine in AWS Step Functions with a blank template.
2. Implement the workflow coordinating the specific execution process of lambda functions. Here is my definition of the state machine:
    ```json
    {
        "Comment": "An AWS Step Functions state machine to extract the words with the highest frequency.",
        "StartAt": "word-frequency-stage",
        "States": {
            "word-frequency-stage": {
                "Type": "Task",
                "Resource": "<FIRST_LAMBDA_ARN>",
                "Next": "FilterHighFrequency"
            },
            "frequent-words-stage": {
                "Type": "Task",
                "Resource": "<SECOND_LAMBDA_ARN>",
                "End": true
            }
        }
    }
    ```
3. Execute the state machine with input to test if the workflow can work correctly.