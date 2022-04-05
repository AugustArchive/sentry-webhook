// 🥅 sentry-webhook: Dead simple webhook worker for Sentry to output events in a Discord channel
// Copyright 2022 Noel <cutie@floofy.dev>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::reply::json;

#[derive(Debug, Serialize, Deserialize)]
struct Empty {}

#[derive(Debug, Serialize, Deserialize)]
struct Response<T>
where
    T: Serialize + Debug,
{
    success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<Error>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Error {
    message: String,
    code: String,
}

impl Error {
    fn new(code: &str, message: &str) -> Error {
        Error {
            message: message.to_string(),
            code: code.to_string(),
        }
    }
}

impl<T> Response<T>
where
    T: Serialize + Debug,
{
    fn new(success: bool, data: T) -> Response<T> {
        Response {
            success,
            data: Some(data),
            errors: None,
        }
    }

    fn err(code: &str, message: &str) -> Response<Empty> {
        Response {
            success: false,
            data: None,
            errors: Some(vec![Error::new(code, message)]),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct IndexResponse {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SentryResponse {}

pub async fn index() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(json(&Response::new(
        true,
        IndexResponse {
            message: "Hello world.".to_string(),
        },
    )))
}

pub async fn sentry(
    _signature: String,
    resource: String,
    payload: HashMap<String, serde_json::Value>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO: do webhook stuff here >:)
    // let stringified_payload = serde_json::to_string(&payload).unwrap();
    // let digested = digest(stringified_payload);

    // if digested != signature {
    //     log::error!("Invalid digest: new={} old={}", digested, signature);
    //     return Ok(json(&Response::<Empty>::err(
    //         "INVALID_SIGNATURE",
    //         "Provided invalid signature.",
    //     )));
    // }

    let action = payload["action"].as_str().unwrap_or("unknown");
    match action {
        "created" => {
            let data = payload["data"].as_object().unwrap();
            let actor = payload["actor"].as_object().unwrap();

            on_action_create(&resource.as_str(), actor, data).await;
            Ok(json(&Response::new(true, SentryResponse {})))
        }
        _ => Ok(json(&Response::<Empty>::err(
            "UNKNOWN_ACTION",
            format!("Unknown action was provided: {}", action).as_str(),
        ))),
    }
}

async fn on_action_create(
    _resource: &str,
    actor: &serde_json::Map<String, serde_json::Value>,
    payload: &serde_json::Map<String, serde_json::Value>,
) {
    // Let's get the error object that was returned
    let error = payload["error"].as_object().unwrap();
    let mut embed = serde_json::json!({
        "title": error["title"].as_str().unwrap_or("???"),
        "color": 0xFF69BD as i32,
        "url": error["web_url"].as_str().unwrap(),
    });

    let empty_vec: Vec<serde_json::Value> = Vec::new();
    let empty_map = serde_json::Map::new();
    let tags = error["tags"].as_array().unwrap_or(&empty_vec);
    // let brebcrumbs = error["breadcrumbs"].as_array();

    let mut description: Vec<String> = vec!["__**Tags**__".to_string()];

    for tag in tags {
        let name = tag[0].as_str().unwrap();
        let value = tag[1].as_str().unwrap();

        let mut tag_value = vec![format!("• **{}**: {}", name, value)];
        description.append(&mut tag_value);
    }

    // Let's build the error
    description.append(&mut vec!["".to_string()]); // append a empty string to create a new line :)

    let exception = error["exception"].as_object().unwrap_or(&empty_map); // If it's an empty map, we don't do anything. :(
    if !exception.is_empty() {
        // Append a codeblock
        description.append(&mut vec![
            "```ts".to_string(),
            error["title"].as_str().unwrap_or("???").to_string(),
        ]);

        // Let's go through the stackframes
        let values = exception["values"].as_array().unwrap_or(&empty_vec);
        for stackframe in values {
            // Get the stacktrace object from this frame
            let trace = stackframe.as_object().unwrap_or(&empty_map);
            let frames = trace["stacktrace"].as_object().unwrap_or(&empty_map)["frames"]
                .as_array()
                .unwrap_or(&empty_vec);

            for frame in frames {
                // Check if we are in app
                if frame["in_app"].as_bool().unwrap_or(false) {
                    let abs_path = frame["abs_path"].as_str().unwrap_or("");

                    // Skip if the absolute path is empty (since we don't really need that >:(
                    if abs_path.len() == 0 {
                        continue;
                    }

                    let path = frame["abs_path"].as_str().unwrap_or("");
                    let column = frame["colno"].as_i64().unwrap_or(-1);
                    let func = frame["function"].as_str().unwrap_or("<anon function>");
                    let line = frame["lineno"].as_i64().unwrap_or(-1);

                    let value = format!("  • {} (path: {}:{}:{})", func, path, line, column);

                    description.append(&mut vec![value.to_string()]);
                }
            }
        }

        description.append(&mut vec!["```".to_string()]);
    }

    embed["description"] = serde_json::Value::String(description.join("\n"));
    // embed["timestamp"] = serde_json::Value::Number(
    //     serde_json::Number::from_f64(error["received"].as_f64().unwrap_or(0.0)).unwrap(),
    // );

    // build out the discord embed
    let value = serde_json::json!({
        "content":
            format!(
                ":pencil2: New issue occurred in project #{} by {} (**{}**)",
                error["project"].as_i64().unwrap(),
                actor["name"].as_str().unwrap(),
                actor["id"].as_str().unwrap()
            ),

        "embeds": vec![embed]
    });

    log::debug!("{}", value);

    // let's just send it lol
    let client = crate::http::CLIENT.clone();
    let webhook_url = crate::config::CONFIG.webhook_url.clone();

    let res = client
        .post(webhook_url)
        .header("content-type", "application/json; charset=utf-8")
        .json(&value)
        .send()
        .await
        .expect("Unable to send request to Discord");

    if res.status().as_u16() != 204 {
        let body = res
            .json::<serde_json::Value>()
            .await
            .expect("Unable to serialize output from Discord");

        log::debug!("{:#?}", body);
    }
}
