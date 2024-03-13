use serde::{Deserialize, Serialize};
use serde_json::json;
use simple_runtime_demo::choose_planet;
use vercel_runtime::{
    http::bad_request, process_request, process_response, run_service, service_fn, Body, Error,
    Request, RequestPayloadExt, Response, ServiceBuilder, StatusCode,
};

/// Represents the payload structure expected in the incoming request.
#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    /// The name included in the payload.
    name: String,
}

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
/// Initializes the logger and runs the service.
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
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        // disable printing the name of the module in every log line.
        .with_target(false)
        .init();

    // This allows to extend the tower service with more layers
    let handler = ServiceBuilder::new()
        .map_request(process_request)
        .map_response(process_response)
        .service(service_fn(handler));

    run_service(handler).await
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
/// use vercel_runtime::{Request, Response, Body, Error};
/// use simple_runtime_demo::handler;
///
/// async fn handle_request(req: Request) -> Result<Response<Body>, Error> {
///     handler(req).await
/// }
/// ```
pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    tracing::info!("Choosing a starter Pokemon");
    let payload = req.payload::<Payload>();

    match payload {
        Err(..) => bad_request(APIError {
            message: "Invalid payload",
            code: "invalid_payload",
        }),
        Ok(None) => bad_request(APIError {
            message: "No payload",
            code: "no_payload",
        }),
        Ok(Some(payload)) => {
            let planet = choose_planet();

            Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(
                json!({
                  "message": format!("{} says: I choose the planet, {}!", payload.name, planet),
                })
                .to_string()
                .into(),
            )?)
        }
    }
}
