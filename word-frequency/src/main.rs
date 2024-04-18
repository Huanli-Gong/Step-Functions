use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use regex::Regex;

#[derive(Deserialize, Serialize)]
struct Input {
    input: String,
}

#[derive(Deserialize, Serialize)]
struct Output {
    data: HashMap<String, usize>
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(process_data);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn process_data(event: Input, _ctx: Context) -> Result<Output, Error> {
    let mut result = HashMap::new();
    let re = Regex::new(r"\b\w+\b").unwrap();

    for word_match in re.find_iter(&event.input) {
        let word = word_match.as_str().to_lowercase();
        let counter = result.entry(word).or_insert(0);
        *counter += 1;
    }

    Ok(Output { data:result })
}
