use flowsnet_platform_sdk::logger;
use lambda_flows::{request_received, send_response};
use openai_flows::{embeddings::EmbeddingsInput, OpenAIFlows};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::str;
use vector_store_flows::*;

static CHAR_SOFT_LIMIT: usize = 9000;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    logger::init();
    request_received(|qry, body| handler(qry, body)).await;
}

async fn handler(qry: HashMap<String, Value>, body: Vec<u8>) {
    let collection_name = qry.get("collection_name").unwrap().as_str().unwrap();
    let vector_size = qry.get("vector_size").unwrap().as_str().unwrap();
    let vector_size: u64 = vector_size.parse().unwrap();
    let mut id: u64 = 0;

    if qry.contains_key("reset") {
        log::debug!("Reset the database");
        // Delete collection, ignore any error
        _ = delete_collection(collection_name).await;
        // Create collection
        let p = CollectionCreateParams { vector_size };
        if let Err(e) = create_collection(collection_name, &p).await {
            log::error!(
                "Cannot create collection named: {} with error: {}",
                collection_name,
                e
            );
            send_success("Cannot create collection");
            return;
        }
    } else {
        log::debug!("Continue with existing database");
        match collection_info(collection_name).await {
            Ok(ci) => {
                id = ci.points_count;
            }
            Err(e) => {
                log::error!("Cannot get collection stat {}", e);
                send_success("Cannot query database!");
                return;
            }
        }
    }
    log::debug!("Starting ID is {}", id);

    let mut openai = OpenAIFlows::new();
    openai.set_retry_times(3);

    let s = str::from_utf8(&body).unwrap();
    let mut points = Vec::<Point>::new();
    let mut current_section = String::new();
    for line in s.lines() {
        if line.starts_with("------") {
            log::debug!("Start openai processing");
            // create and save embedding
            let input = EmbeddingsInput::String(current_section.clone());
            match openai.create_embeddings(input).await {
                Ok(r) => {
                    // log::debug!("Received embedding {:#?}", r);
                    for v in r.iter() {
                        let p = Point {
                            id: PointId::Num(id),
                            vector: v.iter().map(|n| *n as f32).collect(),
                            payload: json!({ "text": current_section })
                                .as_object()
                                .map(|m| m.to_owned()),
                        };
                        points.push(p);
                        log::debug!("Created vector {} with length {}", id, v.len());
                        id += 1;
                    }
                }
                Err(e) => {
                    log::error!("OpenAI returned an error: {}", e);
                }
            }

            // Start a new section
            current_section.clear();
        }
        // Append the line to the current section if the current section is less than CHAR_SOFT_LIMIT
        if current_section.len() < CHAR_SOFT_LIMIT {
            current_section.push_str(line);
            current_section.push('\n');
        } else {
            log::warn!("Section exceeded CHAR_SOFT_LIMIT. Skipped line: {}", line);
        }
    }
    let points_count = points.len();

    if let Err(e) = upsert_points(collection_name, points).await {
        log::error!("Cannot upsert into database! {}", e);
        send_success("Cannot upsert into database!");
        return;
    }

    match collection_info(collection_name).await {
        Ok(ci) => {
            log::debug!(
                "There are {} vectors in collection `{}`",
                ci.points_count,
                collection_name
            );
            send_success(&format!(
                "Successfully inserted {} records. The collection now has {} records in total.",
                points_count, ci.points_count
            ));
        }
        Err(e) => {
            log::error!("Cannot get collection stat {}", e);
            send_success("Cannot upsert into database!");
        }
    }
}

fn send_success(body: &str) {
    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        body.as_bytes().to_vec(),
    );
}
