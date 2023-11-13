use async_openai_wasi::{
    types::{
        CreateMessageRequestArgs, CreateRunRequestArgs, CreateThreadRequestArgs, MessageContent,
    },
    Client,
};
use flowsnet_platform_sdk::logger;
use tg_flows::{listen_to_update, update_handler, Telegram, UpdateKind};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    logger::init();

    create_thread().await;

    let telegram_token = std::env::var("telegram_token").unwrap();
    listen_to_update(telegram_token).await;
}

#[update_handler]
async fn handler(update: tg_flows::Update) {
    logger::init();
    let telegram_token = std::env::var("telegram_token").unwrap();
    let tele = Telegram::new(telegram_token);

    if let UpdateKind::Message(msg) = update.kind {
        let text = msg.text().unwrap_or("");
        let chat_id = msg.chat.id;

        let response = run_message(String::from(text)).await;

        _ = tele.send_message(chat_id, response);
    }
}

async fn create_thread() {
    let client = Client::new();
    let thread_id = std::env::var("WASMEDGE_THREAD_ID").unwrap();

    if let Err(_) = client.threads().retrieve(thread_id.as_str()).await {
        let create_thread_request = CreateThreadRequestArgs::default().build().unwrap();

        match client.threads().create(create_thread_request).await {
            Ok(to) => {
                log::info!("New thread (ID: {}) created.", to.id);
            }
            Err(e) => {
                log::error!("Failed to create thread: {:?}", e);
            }
        }
    }
}

async fn run_message(text: String) -> String {
    let client = Client::new();
    let assistant_id = std::env::var("WASMEDGE_ASSISTANT_ID").unwrap();
    let thread_id = std::env::var("WASMEDGE_THREAD_ID").unwrap();

    let mut create_message_request = CreateMessageRequestArgs::default().build().unwrap();
    create_message_request.content = text;
    let message_object = client
        .threads()
        .messages(&thread_id)
        .create(create_message_request)
        .await
        .unwrap();

    log::debug!("message object: {message_object:#?}");

    let mut create_run_request = CreateRunRequestArgs::default().build().unwrap();
    create_run_request.assistant_id = assistant_id;
    let run_object = client
        .threads()
        .runs(&thread_id)
        .create(create_run_request)
        .await
        .unwrap();

    println!("run object: {run_object:#?}");

    let mut thread_messages = client
        .threads()
        .messages(&thread_id)
        .list(&[("limit", "1")])
        .await
        .unwrap();

    println!("thread messages: {thread_messages:#?}");

    let c = thread_messages.data.pop().unwrap();
    let c = c.content.into_iter().filter_map(|x| match x {
        MessageContent::Text(t) => Some(t.text.value),
        _ => None,
    });

    c.collect()
}
