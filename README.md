# 🥅 Sentry -> Discord Webhook
> *Dead simple webhook worker for Sentry to output events in a Discord channel*

**sentry-webhook** is a simple [Axum](https://github.com/tokio-rs/axum) application to serve as a middle-man from Sentry to Discord to provide more insight about what happened from Sentry.

## Features
- **sentry-webhook** is project-agnostic, if you wish to push only certain projects, just set `SENTRY_PROJECTS=comma,delimited,list` and it will only react on events in `comma,delimited,list`.

- **sentry-webhook** is a dead-simple worker, which means you can deploy it with one command (i.e, `docker run ghcr.io/auguwu/sentry-webhook`). You only need to provide what Sentry server you're using.

## Installation
Before you run **sentry-webhook**, you will need to create a internal application in your Sentry organization. Go to **Settings > Developer Settings > New Internal Integration** and click on "Create New" and fill out whatever, it doesn't matter:

![](https://noel-is.gay/images/e0162290.png)

![](https://noel-is.gay/images/4c892623.png)

Now, you will need to fill in a `.env` file with the following settings:

```env
# The server where your projects live in. This will be used to link where you can view the error at.
SENTRY_SERVER=https://sentry.io

# secret that was generated when the new integration was created. This will be used to validate if Sentry actually
# send that payload or not.
SENTRY_CLIENT_SECRET=
```

The worker will now validate requests that come-in and see if it's from Sentry. The `SENTRY_WEBHOOK_SECRET` should be kept safe, the worker will also validate the payload to check if it actually came from Sentry.

### Locally via Git
To get started running **sentry-webhook** from the repository you are reading from, you will need the following tools:

- [Rust compiler](https://rustlang.org) with the latest version, at the time of writing this, it will be v**1.68.0**, doesn't matter if it is from Stable, Beta, or Nightly channels.

- [Rustup](https://rustup.sh) toolchain with the `cargo` feature
- 512MB of storage
- 512MB of system RAM

To clone the repository into a **sentry-webhook** folder, you can run the `git clone` command:

```sh
$ git clone https://github.com/auguwu/sentry-webhook
```

To build a optimized, production-ready binary, you will need to run `cargo build` with the `--release` flag:

```sh
$ cargo build --release
```

Now you can run **sentry-webhook** with `./target/release/sentry-webhook` (append `.exe` to the end if on Windows).

## License
**sentry-webhook** is released under the **Apache 2.0** License by [Noel](https://floofy.dev)! :polar_bear::purple_heart:
