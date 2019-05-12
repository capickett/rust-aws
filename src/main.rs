extern crate lambda_runtime;
extern crate serde_derive;

use lambda_runtime::{error::HandlerError, lambda, Context};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
struct CustomEvent {
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    number: usize,
}

fn main() {
    lambda!(my_handler);
}

fn my_handler(_e: CustomEvent, _ctx: Context) -> Result<CustomOutput, HandlerError> {
    Ok(CustomOutput {
        number: 1,
    })
}
