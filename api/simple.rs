use serde_json::json;
use simple_runtime_demo::choose_planet;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

/// The main entry point of the application.
///
/// Runs the handler function to handle incoming requests.
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
/// use vercel_runtime::Error;
/// use simple_runtime_demo::handler;
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
/// * `_req` - A `Request` object representing the incoming HTTP request. (Unused in this implementation)
///
/// # Returns
///
/// Returns a `Result` containing either a `Response` if the request is successful, or an `Error` if there's an issue.
///
/// # Examples
///
/// ```rust
/// use vercel_runtime::{Request, Response, Body, Error};
/// use simple_runtime_demo::handler;
///
/// async fn handle_request(req: Request) -> Result<Response<Body>, Error> {
///     handler(req).await
/// }
/// ```
pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let starter = choose_planet();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": format!("I choose the planet, {}!", starter),
            })
            .to_string()
            .into(),
        )?)
}
