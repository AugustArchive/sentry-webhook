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

use std::env::var;
use std::fs::read_to_string;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub webhook_url: String,
    pub sentry_dsn: Option<String>,
    pub level: Option<String>,
    pub port: Option<u16>,
    pub host: Option<String>,
}

fn env_to_config() -> Config {
    // Grab the webhook URL if we have any
    let webhook_url = if let Ok(value) = var("WORKER_WEBHOOK_URL") {
        value
    } else {
        panic!("Missing `WORKER_WEBHOOK_URL` environment variable.");
    };

    let sentry_dsn = var("WORKER_SENTRY_DSN").ok();
    let port = var("WORKER_PORT").ok();
    let host = var("WORKER_HOST").ok();
    let log_level = var("WORKER_LOG_LEVEL").ok();

    Config {
        webhook_url,
        sentry_dsn,
        level: log_level,
        port: port.map(|p| p.parse::<u16>()
                    .expect("Unable to parse String -> u16 for http port.")),
        host,
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let res = read_to_string("config.toml");

    match res {
        Ok(contents) => {
            toml::from_str::<Config>(&contents).expect("Unable to parse 'config.toml' contents.")
        }
        Err(_) => env_to_config(),
    }
});
