# 🥅 sentry-webhook: Dead simple webhook worker for Sentry to output events in a Discord channel
# Copyright 2022 Noel <cutie@floofy.dev>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

FROM rustlang/rust:nightly-alpine3.15 AS builder

RUN apk update && apk add --no-cache build-base openssl-dev gcompat libc6-compat bash
WORKDIR /build/sentry

COPY Cargo.toml .
RUN echo "fn main() {}" >> dummy.rs
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
ENV RUSTFLAGS=-Ctarget-feature=-crt-static
RUN CARGO_INCREMENTAL=1 cargo build --release
RUN rm dummy.rs && sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

# Now we build the actual server
COPY . .
RUN CARGO_INCREMENTAL=1 cargo build --release

FROM alpine:3.17

RUN apk update && apk add --no-cache build-base openssl bash
WORKDIR /app/noel/sentry/webhook
COPY --from=builder /build/sentry/target/release/sentry_webhook .

USER 1001
ENTRYPOINT ["/app/noel/sentry/webhook/sentry_webhook"]
