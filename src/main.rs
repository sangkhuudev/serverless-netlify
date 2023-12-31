use lambda_http::{
    http::header::CONTENT_TYPE, run, service_fn, Body, 
    Request, RequestExt, Response, Error,
};
use serde::Serialize;
use tracing::{info, instrument};

#[derive(Serialize)]
struct ApiResponse {
    data: String,
}

#[instrument]
async fn function_handler(
    event: Request,
) -> Result<Response<Body>, Error> {
    let who = event
    .query_string_parameters_ref()
    .and_then(|param| param.first("name"))
    .unwrap_or("world");
    
    let message = format!(
        "Hello {who}, this is an Netlify serverless request"
    );
    info!(who, message);
    let api_response = ApiResponse { data: message };
    let body_text = serde_json::to_string(&api_response)?;

    let resp = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::Text(body_text))?;

    Ok(resp)
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().json().init();

    run(service_fn(function_handler)).await
}
