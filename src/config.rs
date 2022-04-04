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

use std::fs::read_to_string;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub json_body_max: Option<u64>,
    pub webhook_url: String,
    pub sentry_uri: String,
    pub sentry_dsn: Option<String>,
    pub port: Option<u16>,
    pub host: Option<String>,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let contents = read_to_string("config.toml").expect("Unable to open config.toml :(");
    toml::from_str(&contents).unwrap()
});
