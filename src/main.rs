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

use ansi_term::Colour::RGB;
use chrono::Local;
use fern::Dispatch;
use log::{debug, info};
use std::{collections::HashMap, env::var, net::SocketAddr, thread};
use warp::Filter;

mod config;
mod http;
mod routes;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging();
    setup_sentry();

    info!("starting up worker...");
    let index = warp::get().and(warp::path!()).and_then(routes::index);

    let sentry = warp::post()
        .and(warp::path!("sentry"))
        .and(warp::header::<String>("sentry-hook-signature"))
        .and(warp::header::<String>("sentry-hook-resource"))
        .and(warp::body::json::<HashMap<String, serde_json::Value>>())
        .and_then(routes::sentry);

    let port = config::CONFIG.port.unwrap_or(3939);
    let routes = warp::any()
        .and(index.or(sentry))
        .with(warp::log("sentry::http"));

    // Check if we can parse the config host to a SocketAddr.
    let host = config::CONFIG.host.clone();
    let addr = if let Some(h) = host {
        format!("{}:{}", h, port)
            .parse::<SocketAddr>()
            .unwrap_or_else(|_| panic!("Unable to parse host '{}' -> SocketAddr.", h))
    } else {
        format!("{}:{}", "0.0.0.0", port)
            .parse::<SocketAddr>()
            .unwrap()
    };

    warp::serve(routes).run(addr).await;
    Ok(())
}

fn setup_logging() {
    let raw_level = config::CONFIG.level.clone();
    let level = raw_level.unwrap_or_else(|| "info".to_string());

    let log_level = match level.as_str() {
        "off" => log::LevelFilter::Off,
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Info,
    };

    let dispatch = Dispatch::new()
        .format(|out, message, record| {
            let current_thread = thread::current();
            let name = current_thread.name().unwrap_or("main");

            if var("SENTRY_WORKER_DISABLE_COLORS").is_ok() {
                out.finish(format_args!(
                    "{} [{} ({})] {} :: {}",
                    Local::now().format("[%B %d, %G | %H:%M:%S %p]"),
                    record.target(),
                    name,
                    record.level(),
                    message
                ));
            } else {
                let now = Local::now().format("[%B %d, %G | %H:%M:%S %p]");
                let color = match record.level() {
                    log::Level::Error => RGB(153, 75, 104),
                    log::Level::Debug => RGB(163, 182, 138),
                    log::Level::Info => RGB(178, 157, 243),
                    log::Level::Trace => RGB(163, 182, 138),
                    log::Level::Warn => RGB(243, 243, 134),
                };

                out.finish(format_args!(
                    "{} {}{}{} {} :: {}",
                    RGB(134, 134, 134).paint(format!("{}", now)),
                    RGB(178, 157, 243).paint(format!("[{} ", record.target())),
                    RGB(255, 105, 189).paint(format!("({})", name)),
                    RGB(178, 157, 243).paint("]"),
                    color.paint(format!("{}", record.level())),
                    message
                ));
            }
        })
        .level(log_level)
        .chain(std::io::stdout());

    if let Err(error) = dispatch.apply() {
        panic!("{}", error);
    }
}

fn setup_sentry() {
    let dsn = config::CONFIG.sentry_dsn.clone();
    if let Some(dsn_uri) = dsn {
        debug!("Enabling Sentry with DSN '{}'...", dsn_uri);
    }
}
