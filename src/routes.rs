// 🥅 sentry-webhook: Webhook handler to output Sentry errors into a Discord channel.
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
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::reply::json;

#[derive(Debug, Serialize, Deserialize)]
struct Response<T>
where
    T: Serialize + Debug,
{
    success: bool,
    data: T,
}

impl<T> Response<T>
where
    T: Serialize + Debug,
{
    fn new(success: bool, data: T) -> Response<T> {
        Response { success, data }
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
    payload: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Get the action that occurred
    debug!("received data: {:#?}", payload);

    Ok(json(&Response::new(true, SentryResponse {})))
}
