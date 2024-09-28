use aes_gcm::{Aes256Gcm, Key};
use axum::routing::post;
use axum::{Extension, Json, Router};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use encryption::aes_gcm::decrypt;
use encryption::encryption_codec::fetch_key;
use prost::Message;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use temporal_sdk_core_protos::temporal::api::common::v1::Payload;
use temporal_sdk_core_protos::JSON_ENCODING_VAL;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let key = Arc::new(fetch_key("test").await);

    let app = Router::new()
        .route(
            "/decode",
            post({
                let shared_key = key.clone();
                move |body: Json<serde_json::Value>| decode(body, shared_key)
            }),
        )
        .layer(Extension(key))
        .layer(
            CorsLayer::new()
                .allow_headers([
                    "x-namespace".parse().unwrap(),
                    "content-type".parse().unwrap(),
                ])
                .allow_origin(Any),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8888")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn decode(
    Json(s): Json<serde_json::Value>,
    key: Arc<Key<Aes256Gcm>>,
) -> Json<ToJsonPayloads> {
    println!("Decoding payload: {:?}", s);
    let base64 = s
        .as_object()
        .unwrap()
        .get("payloads")
        .unwrap()
        .as_array()
        .unwrap()
        .first()
        .unwrap()
        .as_object()
        .unwrap()
        .get("data")
        .unwrap()
        .as_str()
        .unwrap();

    let data = BASE64_STANDARD.decode(base64).unwrap();
    let new_data = decrypt(&data, &key).unwrap();
    let new_data = Payload::decode(new_data.as_slice()).unwrap().data;

    // bytes must be return to base64 encoded string
    Json(
        ToJsonPayload {
            metadata: HashMap::from([(
                "encoding".to_string(),
                BASE64_STANDARD.encode(JSON_ENCODING_VAL),
            )]),
            data: BASE64_STANDARD.encode(&new_data),
        }
        .into(),
    )
}

#[derive(Serialize)]
struct ToJsonPayload {
    metadata: HashMap<String, String>,
    data: String,
}

#[derive(Serialize)]
struct ToJsonPayloads {
    payloads: Vec<ToJsonPayload>,
}

impl From<ToJsonPayload> for ToJsonPayloads {
    fn from(payload: ToJsonPayload) -> Self {
        ToJsonPayloads {
            payloads: vec![payload],
        }
    }
}
