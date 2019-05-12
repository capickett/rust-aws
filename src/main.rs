extern crate lambda_runtime;
extern crate serde_derive;
extern crate simple_error;

use lambda_runtime::{error::HandlerError, lambda, Context};
use serde_derive::{Deserialize, Serialize};
use simple_error::bail;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    first_name: String,
    last_name: Option<String>,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn main() {
    lambda!(my_handler);
}

fn my_handler(e: CustomEvent, _ctx: Context) -> Result<CustomOutput, HandlerError> {
    if e.first_name.is_empty() {
        bail!("Empty first name!");
    }

    Ok(CustomOutput {
        message: format!("Hello, {}!", e.first_name),
    })
}
