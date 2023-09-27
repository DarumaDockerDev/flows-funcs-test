use discord_flows::{application_command_handler, message_handler};
use discord_flows::{
    model::{
        application::interaction::InteractionResponseType,
        prelude::application::interaction::application_command::ApplicationCommandInteraction,
        Message,
    },
    Bot, ProvidedBot,
};
use flowsnet_platform_sdk::logger;
use http_req::request;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use webhook_flows::{create_endpoint, request_handler, send_response};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    logger::init();
    let token = std::env::var("DISCORD_TOKEN").unwrap();

    let bot = ProvidedBot::new(token);
    bot.listen_to_application_commands_from_channel(
        std::env::var("LISTENING_DISCORD_CHANNEL_ID")
            .unwrap()
            .parse()
            .unwrap(),
    )
    .await;
    bot.listen_to_messages_from_channel(
        std::env::var("LISTENING_DISCORD_CHANNEL_ID")
            .unwrap()
            .parse()
            .unwrap(),
    )
    .await;

    create_endpoint().await;
}

#[message_handler]
pub async fn message_handler(msg: Message) {
    logger::init();
    let token = std::env::var("DISCORD_TOKEN").unwrap();

    let bot = ProvidedBot::new(token);
    let client = bot.get_client();
    let channel_id = msg.channel_id;
    let content = msg.content;

    if msg.author.bot {
        log::debug!("message from bot");
        return;
    }

    _ = client
        .send_message(
            channel_id.into(),
            &serde_json::json!({
                "content": content,
            }),
        )
        .await;
}

#[application_command_handler]
pub async fn application_command_handler(ac: ApplicationCommandInteraction) {
    logger::init();
    let token = std::env::var("DISCORD_TOKEN").unwrap();
    let bot = ProvidedBot::new(token);

    let client = bot.get_client();
    log::debug!("-------------------");

    let x = client
        .create_interaction_response(
            ac.id.into(),
            &ac.token,
            &serde_json::json!({
                "type": InteractionResponseType::DeferredChannelMessageWithSource as u8,
            }),
        )
        .await;
    log::debug!("{:?}", x);
    tokio::time::sleep(Duration::from_secs(3)).await;
    client.set_application_id(ac.application_id.into());
    _ = client
        .edit_original_interaction_response(
            &ac.token,
            &serde_json::json!({
                "content": "Pong"
            }),
        )
        .await;

    if let Ok(m) = client
        .create_followup_message(
            &ac.token,
            &serde_json::json!({
                "content": "PongPong"
            }),
        )
        .await
    {
        _ = client
            .edit_followup_message(
                &ac.token,
                m.id.into(),
                &serde_json::json!({
                    "content": "PongPongPong"
                }),
            )
            .await;
    }
}

#[request_handler]
async fn webhook_handler(
    _headers: Vec<(String, String)>,
    subpath: String,
    qry: HashMap<String, Value>,
    _body: Vec<u8>,
) {
    // thread::sleep(Duration::from_secs(10));
    println!("{:?}", subpath);

    let city = qry.get("city").unwrap_or(&Value::Null).as_str();
    let resp = match city {
        Some(c) => get_weather(c).map(|w| {
            format!(
                "Today: {},
Low temperature: {} °C,
High temperature: {} °C,
Wind Speed: {} km/h",
                w.weather
                    .first()
                    .unwrap_or(&Weather {
                        main: "Unknown".to_string()
                    })
                    .main,
                w.main.temp_min as i32,
                w.main.temp_max as i32,
                w.wind.speed as i32
            )
        }),
        None => Err(String::from("No city in query")),
    };

    match resp {
        Ok(r) => send_response(
            200,
            vec![(
                String::from("content-type"),
                String::from("text/html; charset=UTF-8"),
            )],
            r.as_bytes().to_vec(),
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

#[derive(Deserialize)]
struct ApiResult {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
}

#[derive(Deserialize)]
struct Weather {
    main: String,
}

#[derive(Deserialize)]
struct Main {
    temp_max: f64,
    temp_min: f64,
}

#[derive(Deserialize)]
struct Wind {
    speed: f64,
}

fn get_weather(city: &str) -> Result<ApiResult, String> {
    let mut writer = Vec::new();
    let api_key = "09a55b004ce2f065b903015e3284de35";
    let query_str = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={city}&units=metric&appid={api_key}"
    );

    request::get(query_str, &mut writer)
        .map_err(|e| e.to_string())
        .and_then(|_| {
            serde_json::from_slice::<ApiResult>(&writer).map_err(|_| {
                "Please check if you've typed the name of your city correctly".to_string()
            })
        })
}
