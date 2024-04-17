use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Input {
    data: String,
}

#[derive(Deserialize, Serialize)]
struct Output {
    longest_word: String,
    original_text: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(find_longest_word);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn find_longest_word(event: Input, _ctx: Context) -> Result<Output, Error> {
    let mut max_length = 0;
    let mut longest_word = String::new();

    for word in event.data.split_whitespace() {
        let cleaned_word = word.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect::<String>();
        if cleaned_word.len() > max_length {
            max_length = cleaned_word.len();
            longest_word = cleaned_word;
        }
    }

    Ok(Output {
        longest_word,
        original_text: event.data,
    })
}

