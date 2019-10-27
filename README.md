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
