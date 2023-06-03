use std::collections::HashMap;

use flowsnet_platform_sdk::logger;
use lambda_flows::{request_received, send_response};
use serde_json::{json, Value};

use vector_store_flows::*;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    logger::init();
    request_received(handler).await;
}

async fn handler(_qry: HashMap<String, Value>, _body: Vec<u8>) {
    let collection_name = "test";

    // Delete collection
    _ = delete_collection(collection_name).await;

    // Create and get collection
    {
        let p = CollectionCreateParams { vector_size: 4 };
        if let Err(_) = create_collection(collection_name, &p).await {
            return;
        }

        match collection_info(collection_name).await {
            Ok(ci) => {
                log::debug!(
                    "There are {} vectors in collection `{}` just when created",
                    ci.points_count,
                    collection_name
                );
            }
            Err(_) => {
                return;
            }
        }
    }

    // Upsert points
    {
        let p = vec![
            Point {
                id: PointId::Num(1),
                vector: vec![0.05, 0.61, 0.76, 0.74],
                payload: Some(json!({
                    "city": "Berlin",
                    "country": "Germany",
                    "count": 1000000,
                    "square": 12.5,
                    "coords": {"lat": 1.0, "lon": 2.0},
                })),
            },
            Point {
                id: PointId::Num(2),
                vector: vec![0.19, 0.81, 0.75, 0.11],
                payload: Some(json!({
                    "city": ["Berlin", "London"],
                })),
            },
            Point {
                id: PointId::Num(3),
                vector: vec![0.36, 0.55, 0.47, 0.94],
                payload: Some(json!({
                    "city": ["Berlin", "Moscow"],
                })),
            },
            Point {
                id: PointId::Num(4),
                vector: vec![0.18, 0.01, 0.85, 0.8],
                payload: Some(json!({
                    "city": ["London", "Moscow"],
                })),
            },
            Point {
                id: PointId::Uuid(String::from("98a9a4b1-4ef2-46fb-8315-a97d874fe1d7")),
                vector: vec![0.24, 0.18, 0.22, 0.44],
                payload: Some(json!({
                    "count": [0],
                })),
            },
            Point {
                id: PointId::Uuid(String::from("f0e09527-b096-42a8-94e9-ea94d342b925")),
                vector: vec![0.35, 0.08, 0.11, 0.44],
                payload: None,
            },
        ];

        if let Err(_) = upsert_points(collection_name, p).await {
            return;
        }

        log::debug!("Points has been upserted.");
    }

    // Search points
    {
        let p = PointsSearchParams {
            vector: vec![0.2, 0.1, 0.9, 0.7],
            limit: 3,
        };

        match search_points(collection_name, &p).await {
            Ok(sp) => send_response(
                200,
                vec![(
                    String::from("content-type"),
                    String::from("text/html; charset=UTF-8"),
                )],
                serde_json::to_vec_pretty(&sp).unwrap(),
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
}
