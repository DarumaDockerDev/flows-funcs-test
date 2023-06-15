use dotenv::dotenv;
use openai_flows::{
    chat::{ChatModel, ChatOptions},
    OpenAIFlows,
};
use slack_flows::{listen_to_channel, send_message_to_channel, SlackMessage};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();
    listen_to_channel("family-wangshi", "chat", |sm| handler(sm)).await;
}

async fn handler(sm: SlackMessage) {
    let mut openai = OpenAIFlows::new();
    openai.set_retry_times(3);

    let co = ChatOptions {
        model: ChatModel::GPT35Turbo,
        restart: false,
        system_prompt: None,
    };

    match openai
        .chat_completion("any_conversation", &sm.text, &co)
        .await
    {
        Ok(r) => {
            send_message_to_channel("family-wangshi", "chat", r.choice).await;
        }
        Err(e) => {
            send_message_to_channel("family-wangshi", "chat", e.to_string()).await;
        }
    }
}
