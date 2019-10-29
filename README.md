[![CircleCI](https://circleci.com/gh/sinsoku/rust_web.svg?style=svg&circle-token=f2d7ec9fce88998e3c6b551184ef8c4ab2b7ab8d)](https://circleci.com/gh/sinsoku/rust_web)

# Web App with Rust

## Setup

This app uses [diesel](https://diesel.rs/), so you need to install it.

```console
$ cargo install diesel_cli --no-default-features --features sqlite
$ export DATABASE_URL=development.sqlite3
$ diesel setup
```

## Run application

You can launch the app by `cargo run`.

```console
$ cargo run
```

Then browsing http://localhost:8088, you see "Hello, world!"
