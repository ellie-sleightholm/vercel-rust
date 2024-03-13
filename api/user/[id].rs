use serde::Serialize;
use std::collections::HashMap;
use url::Url;
use vercel_runtime::{http::bad_request, run, Body, Error, Request, Response, StatusCode};

/// Represents an error response in the API.
#[derive(Serialize)]
pub struct APIError {
    /// A message describing the error.
    pub message: &'static str,
    /// A code representing the error.
    pub code: &'static str,
}

/// The main entry point of the application.
///
/// # Returns
///
/// Returns `Ok(())` if the application runs successfully.
///
/// # Errors
///
/// Returns an error if there's an issue during runtime.
///
/// # Examples
///
/// ```rust
/// use crate::handler;
/// use vercel_runtime::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     handler().await?;
///     Ok(())
/// }
/// ```
#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

/// Handles incoming requests to the API.
///
/// # Arguments
///
/// * `req` - A `Request` object representing the incoming HTTP request.
///
/// # Returns
///
/// Returns a `Result` containing either a `Response` if the request is successful, or an `Error` if there's an issue.
///
/// # Examples
///
/// ```rust
/// use crate::handler;
/// use vercel_runtime::{Request, Response, Body, Error};
///
/// async fn handle_request(req: Request) -> Result<Response<Body>, Error> {
///     handler(req).await
/// }
/// ```
pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let parsed_url = Url::parse(&req.uri().to_string()).unwrap();
    let hash_query: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();
    let id_key = hash_query.get("id");

    match id_key {
        None => {
            return bad_request(APIError {
                message: "Query string is invalid",
                code: "query_string_invalid",
            });
        }
        Some(id) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::Text(id.to_owned()))?),
    }
}
