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
    let co = ChatOptions {
        model: Some("gpt-3.5-turbo"),
        token_limit: 4096,
        ..Default::default()
    };
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();
    let lf = LLMServiceFlows::new(
        "https://api.openai.com/v1/chat/completions",
        api_key.as_str(),
    );

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
