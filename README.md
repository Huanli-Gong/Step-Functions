[![pipeline status](https://gitlab.com/dukeaiml/IDS721/zhankai_ye_mini_project_3/-/wikis/uploads/8b9be60c3dabf89331bc0c118cae1e44/pipeline.svg)](https://gitlab.com/dukeaiml/IDS721/zhankai_ye_individual_project_4/-/commits/main)
# Rust AWS Lambda and Step Functions for Simple Text Analysis
This project aims to develope a text data processing pipeline using two orchestrated lambda function: the `longest_word_identification` function identifies the longest word in the input sentence and the `counting_occurrences` function counts the occurrence of the longest words. The two lambda functions are coordinated in the State Machine of AWS Step Funtions.

## Goals
* Rust AWS Lambda function
* Step Functions workflow coordinating Lambdas
* Orchestrate data processing pipeline

## Steps
### Step 1: Create Lambda Functions
1. Initializing new AWS Lambda project in Rust using command line `cargo lambda new <PROJECT_NAME>` in terminal.
```
cargo lambda new longest_word_identification
```
```
cargo lambda new counting_occurrences
```
2. Add necessary dependencies to `Cargo.toml` file.
3. Add functional implementations and inference endpoint in `main.rs` file.
4. Create the .json files and put at their corresponding project directory for local testing:
* input.json
```
{"data": "The brown fox quickly jumps over the lazy dog, then the dog flees away quickly."}
```
* input2.json
```
{"longest_word":"quickly","original_text":"The brown fox quickly jumps over the lazy dog, then the dog flees away quickly."}
```
5. Test these two lambda functions locally by running:
```
cd longest_word_identification
cargo lambda watch
cargo lambda invoke --data-file input.json
```
```
cd counting_occurrences
cargo lambda watch
cargo lambda invoke --data-file input2.json
```
### Step 2: Deploy Lambda Functions to AWS
1. After succesfully testing the lambda functions, push these functions to the Gitlab.
2. Set the AWS access variable in `Settings` -> `CI/CD`.
3. Add the `.gitlab-ci.yml` file. Then, it will automatically deploy the lambda functions to AWS.

### Step 3: Orchestrate Step Functions Pipeline
1. Open the AWS Management Console and navigate to the AWS Step Functions page.
2. Create a new state machine in AWS Step Functions. Choose `AWS Lambda Invoke`.
3. Define the state machine as follows:
```
{
  "Comment": "A simple AWS Step Functions state machine that coordinates two lambda functions.",
  "StartAt": "longest_word_identification",
  "States": {
    "longest_word_identification": {
      "Type": "Task",
      "Resource": "arn:aws:lambda:us-east-1:590183895316:function:longest_word_identification",
      "Next": "counting_occurrences"
    },
    "counting_occurrences": {
      "Type": "Task",
      "Resource": "arn:aws:lambda:us-east-1:590183895316:function:counting_occurrences",
      "End": true
    }
  }
}
```
4. Click `Start Execution` to test the pipeline with the input:
```
{"data": "The brown fox quickly jumps over the lazy dog, then the dog flees away quickly."}
```

## Results
### Lambda Functions
* longest_word_identification
![lambda_1](https://gitlab.com/dukeaiml/IDS721/step_functions/-/wikis/uploads/9b766741431119fa21ec18fa265290f6/lambda_1.png)
* counting_occurrences
![lambda_2](https://gitlab.com/dukeaiml/IDS721/step_functions/-/wikis/uploads/4803dbf93024d2968a5ef1a79021658f/lambda_2.png)

### State Machine
![state_machine_1](https://gitlab.com/dukeaiml/IDS721/step_functions/-/wikis/uploads/ddfd04635821f3f70ca6b5253c04bc54/state_machine_1.png)
* Execution Graph
![state_machine_2](https://gitlab.com/dukeaiml/IDS721/step_functions/-/wikis/uploads/31d4537c12f37c49d992aa2cee245b09/state_machine_2.png)
* Log showing succesfull deployment and execution
![state_machine_3](https://gitlab.com/dukeaiml/IDS721/step_functions/-/wikis/uploads/a82b7ebf59dd23c4a5f778392a2f4c19/state_machine_3.png)

## Demo
![demo_individual_4](https://gitlab.com/dukeaiml/IDS721/step_functions/-/wikis/uploads/46617b0e1df56e6ae019b706a4719647/demo_individual_4.mov)