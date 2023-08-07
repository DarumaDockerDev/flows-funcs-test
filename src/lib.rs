use std::collections::HashMap;

use llmservice_flows::{chat::ChatOptions, LLMServiceFlows};

use lambda_flows::{request_received, send_response};
use serde_json::Value;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    request_received(handler).await;
}

async fn handler(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, body: Vec<u8>) {
    let model = std::env::var("LLM_SERVICE_MODEL").ok();
    let co = ChatOptions {
        model: model.as_deref(),
        token_limit: 4096,
        ..Default::default()
    };
    let endpoint = std::env::var("LLM_SERVICE_ENDPOINT").unwrap();
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();
    let mut lf = LLMServiceFlows::new(endpoint.as_str());
    lf.set_api_key(api_key.as_str());

    let r = match lf
        .chat_completion(
            "example_conversion_1",
            String::from_utf8_lossy(&body).into_owned().as_str(),
            &co,
        )
        .await
    {
        Ok(c) => c.choice,
        Err(e) => e,
    };

    send_response(
        200,
        vec![(
            String::from("content-type"),
            String::from("text/plain; charset=UTF-8"),
        )],
        r.as_bytes().to_vec(),
    );
}
