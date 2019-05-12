extern crate lambda_runtime;
extern crate serde_derive;

use lambda_runtime::{error::HandlerError, lambda, Context, Handler};
use serde_derive::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Deserialize)]
#[serde(untagged)]
enum CounterRequest {
    Set { number: usize },
    Get {},
}

#[derive(Serialize, Clone)]
#[serde(untagged)]
enum CounterResponse {
    Get { number: usize },
    Set { number: usize, message: String },
}

#[derive(Default)]
struct CounterHandler {
    number: AtomicUsize,
}

impl CounterHandler {
    fn get_counter(&self) -> Result<CounterResponse, HandlerError> {
        Ok(CounterResponse::Get {
            number: self.number.load(Ordering::Relaxed),
        })
    }

    fn set_counter(&mut self, number: usize) -> Result<CounterResponse, HandlerError> {
        self.number.store(number, Ordering::Release);
        Ok(CounterResponse::Set {
            number: number,
            message: "Success!".to_string(),
        })
    }
}

impl Handler<CounterRequest, CounterResponse, HandlerError> for CounterHandler {
    fn run(&mut self, req: CounterRequest, _ctx: Context) -> Result<CounterResponse, HandlerError> {
        match req {
            CounterRequest::Get {} => self.get_counter(),
            CounterRequest::Set { number: num } => self.set_counter(num),
        }
    }
}

fn main() {
    let handler = CounterHandler::default();
    lambda!(handler);
}
