# 🥅 Sentry Webhook
> *Dead simple webhook worker for Sentry to output events in a Discord channel*

## Why?
This is just a simple **Rust** HTTP service to do so, this is just for my use case and no one else. :)

## Installation
You just need **Rust**, the version I am using (at the time of writing this is):

```sh
$ rustc --version
# rustc 1.61.0-nightly (1eb72580d 2022-03-08)
```

```sh
$ git clone https://github.com/auguwu/sentry-webhook
$ cargo build --release
$ ./target/release/sentry_webhook
```

### Docker
You can use the Docker image from the **GitHub Container Registry**:

```shell
$ docker run -d -p 3939:3939 --name sentry-worker -v /path/to/config.toml:/app/noel/sentry/worker/config.toml ghcr.io/auguwu/sentry-worker:latest # or prepend a version :>
```

## License
**sentry-webhook** is released under the **Apache 2.0** License by Noel.
