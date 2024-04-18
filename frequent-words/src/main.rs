use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

#[derive(Deserialize, Serialize)]
struct Input {
    data: HashMap<String, usize>,
}

#[derive(Deserialize, Serialize)]
struct Output {
    result: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(filter_top_words);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn filter_top_words(event: Input, _ctx: Context) -> Result<Output, Error> {
    let word_counts = event.data;

    let total_words = word_counts.len();
    let top_k_size = (total_words as f32 * 0.1).ceil() as usize;
    let mut min_heap = BinaryHeap::with_capacity(top_k_size + 1);

    for (word, count) in word_counts {
        min_heap.push(Reverse((count, word.clone())));

        if min_heap.len() > top_k_size {
            min_heap.pop();
        }
    }

    let mut result = Vec::with_capacity(top_k_size);
    while let Some(Reverse((_, word))) = min_heap.pop() {
        result.push(word);
    }

    result.reverse();
    Ok(Output { result })
}
