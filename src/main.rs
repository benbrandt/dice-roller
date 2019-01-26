use std::error::Error;

use lambda_http::{lambda, IntoResponse, Request, RequestExt, Response};
use lambda_runtime::{error::HandlerError, Context};
use log::{self, error};
use serde_json::json;
use simple_logger;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(handler);
    Ok(())
}

fn handler(event: Request, ctx: Context) -> Result<impl IntoResponse, HandlerError> {
    Ok(match event.query_string_parameters().get("dice") {
        Some(dice) => match dice_roller::roll(dice) {
            Ok(roll) => json!({ "roll": roll.to_string() }).into_response(),
            Err(m) => {
                error!("Invalid dice in request {}", ctx.aws_request_id);
                Response::builder()
                    .status(400)
                    .body(json!({ "message": m }).to_string().into())
                    .expect("failed to render response")
            }
        },
        _ => {
            error!("Empty dice in request {}", ctx.aws_request_id);
            Response::builder()
                .status(400)
                .body("Empty dice".into())
                .expect("failed to render response")
        }
    })
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn handler_handles() {
//         let request = Request::default();
//         let expected = json!({
//         "message": "Go Serverless v1.0! Your function executed successfully!"
//         })
//         .into_response();
//         let response = handler(request, Context::default())
//             .expect("expected Ok(_) value")
//             .into_response();
//         assert_eq!(response.body(), expected.body())
//     }
// }
