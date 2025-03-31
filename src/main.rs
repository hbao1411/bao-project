use lambda_runtime::{Error, LambdaEvent, service_fn};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Deserialize)]
struct Payload {
    body: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Msg {
    msg: String,
}

#[derive(Deserialize)]
struct RequestName {
    name: String,
}

#[derive(Serialize)]
struct Response {
    #[serde(rename = "statusCode")]
    status: u16,
    headers: Value,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = service_fn(func);
    lambda_runtime::run(handler).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Payload>) -> Result<Value, Error> {
    let body = event.payload.body.unwrap_or("Unknown".to_string());
    let req_name: Option<RequestName> = serde_json::from_str(&body).ok();

    if let Some(req_name) = req_name {
        let resp = Response {
            status: 200,
            headers: json!({ "Content-Type": "application/json" }),
            body: json!(Msg {
                msg: format!("Hello {} from AWS lambda!", req_name.name),
            })
            .to_string(),
        };
        let resp = serde_json::to_value(&resp)?;

        return Ok(resp);
    }
    // add new comment for test Jenkins 2 - tEST ver 2
    let resp = Response {
        status: 400,
        headers: json!({ "Content-Type": "application/json" }),
        body: json!(Msg {
            msg: "bad request from client".to_string(),
        })
        .to_string(),
    };
    let resp = serde_json::to_value(&resp)?;

    Ok(resp)
}
