#!/usr/bin/env just --justfile

set dotenv-load

default := invocation_directory()

alias server:=worker

@fmt:
  cargo fmt --check

@temporal:
  temporal server start-dev

@worker crate=default:
  cargo run --manifest-path {{crate}}/Cargo.toml --bin main

@client crate=default:
  cargo run --manifest-path {{crate}}/Cargo.toml --bin client
