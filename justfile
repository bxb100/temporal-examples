#!/usr/bin/env just --justfile

set dotenv-load

default := invocation_directory()


@rustfmt:
  find {{invocation_directory()}} -name \*.rs -exec rustfmt {} \;

@temporal:
  temporal server start-dev

@server crate=default:
  cargo run --manifest-path {{crate}}/Cargo.toml

@client crate=default:
  cargo run --manifest-path {{crate}}/Cargo.toml --bin client
