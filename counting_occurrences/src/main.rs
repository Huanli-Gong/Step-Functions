use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
struct Input {
    longest_word: String,
    original_text: String,
}

#[derive(Deserialize, Serialize)]
struct Output {
    longest_word: String,
    occurrences: usize,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(count_occurrences);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn count_occurrences(event: Input, _ctx: Context) -> Result<Output, Error> {
    let mut word_map = HashMap::new();

    for word in event.original_text.split_whitespace() {
        let cleaned_word = word.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect::<String>();
        let count = word_map.entry(cleaned_word).or_insert(0);
        *count += 1;
    }

    let occurrences = *word_map.get(&event.longest_word).unwrap_or(&0);

    Ok(Output {
        longest_word: event.longest_word,
        occurrences,
    })
}

