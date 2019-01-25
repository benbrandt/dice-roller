extern crate lambda_runtime as lambda;
extern crate serde_derive;

use lambda::{error::HandlerError, lambda, Context};
use serde_derive::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct RollEvent {
    dice: String,
}

#[derive(Serialize, Deserialize)]
struct RollResponse {
    roll: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(handler);
    Ok(())
}

fn handler(event: RollEvent, ctx: Context) -> Result<RollResponse, HandlerError> {
    if event.dice == "" {
        return Err(ctx.new_error("No dice"));
    }

    match dice_roller::roll(&event.dice) {
        Ok(r) => Ok(RollResponse {
            roll: r.to_string(),
        }),
        Err(m) => Err(ctx.new_error(m)),
    }
}
