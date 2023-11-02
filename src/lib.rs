use std::collections::HashMap;

use serde_json::Value;
use webhook_flows::{create_endpoint, request_handler, send_response};

use twilio_wasi::{Client, OutboundMessage};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    create_endpoint().await;
}

#[request_handler(GET, POST)]
async fn handler(
    _headers: Vec<(String, String)>,
    subpath: String,
    qry: HashMap<String, Value>,
    _body: Vec<u8>,
) {
    flowsnet_platform_sdk::logger::init();

    let text = qry.get("text").unwrap_or(&Value::Null).as_str();
    let to = qry.get("to").unwrap_or(&Value::Null).as_str();
    let resp = match text.and(to) {
        Some(_) => {
            let account_id = std::env::var("TWILIO_ACCOUNT_ID").unwrap();
            let auth_token = std::env::var("TWILIO_AUTH_TOKEN").unwrap();
            let from = std::env::var("TWILIO_FROM").unwrap();
            let client = Client::new(account_id.as_str(), auth_token.as_str());
            client
                .send_message(OutboundMessage::new(
                    from.as_str(),
                    to.unwrap(),
                    text.unwrap(),
                ))
                .await
                .map_err(|x| format!("{:?}", x))
        }
        None => Err(String::from("No 'text' or 'to' in query")),
    };

    match resp {
        Ok(r) => send_response(
            200,
            vec![(
                String::from("content-type"),
                String::from("text/html; charset=UTF-8"),
            )],
            format!("{:?}", r).as_bytes().to_vec(),
        ),
        Err(e) => send_response(
            400,
            vec![(
                String::from("content-type"),
                String::from("text/html; charset=UTF-8"),
            )],
            e.as_bytes().to_vec(),
        ),
    }
}
