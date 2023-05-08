use flowsnet_platform_sdk::logger;
use lambda_flows::{request_received, send_response};
use openai_flows::{
    chat::{ChatModel, ChatOptions},
    OpenAIFlows,
};
use serde_json::Value;
use std::collections::HashMap;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    logger::init();
    request_received(handler).await;
}

async fn handler(_qry: HashMap<String, Value>, body: Vec<u8>) {
    let co = ChatOptions {
        model: ChatModel::GPT35Turbo,
        restart: false,
        system_prompt: None,
    };
    let of = OpenAIFlows::new();

    let r = match of
        .chat_completion(
            "any_conversation_id",
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
