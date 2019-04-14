use std::error::Error;

use http::{header::CONTENT_TYPE, StatusCode};
use lambda_http::{lambda, Body, Request, RequestExt, Response};
use lambda_runtime::{error::HandlerError, Context};
use log::error;
use sentry;
use serde_json::json;

fn main() -> Result<(), Box<dyn Error>> {
    let _guard = sentry::init("https://046b94f8170f4135a47ca9d0f9709a6d@sentry.io/1438468");
    sentry::integrations::env_logger::init(None, Default::default());
    lambda!(handler);
    Ok(())
}

fn handler(event: Request, ctx: Context) -> Result<Response<Body>, HandlerError> {
    let mut response = Response::builder();
    response
        .header("Access-Control-Allow-Origin", "*")
        .header(CONTENT_TYPE, "application/json");

    Ok(match event.query_string_parameters().get("dice") {
        Some(dice) => match dice_roller::roll(dice) {
            Ok(roll) => response
                .status(StatusCode::OK)
                .body(json!(roll).to_string().into())
                .expect("failed to render response"),
            Err(m) => {
                error!("Invalid dice in request {}", ctx.aws_request_id);
                response
                    .status(StatusCode::BAD_REQUEST)
                    .body(json!({ "message": m }).to_string().into())
                    .expect("failed to render response")
            }
        },
        _ => {
            error!("Empty dice in request {}", ctx.aws_request_id);
            response
                .status(400)
                .body(json!({ "message": "Empty dice" }).to_string().into())
                .expect("failed to render response")
        }
    })
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
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
