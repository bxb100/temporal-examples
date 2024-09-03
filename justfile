#!/usr/bin/env just --justfile

set dotenv-load

path := invocation_directory()

alias c:=client
alias server:=worker
alias s:=worker

default:
  @echo {{path}}

@fmt:
  cargo fmt --check

@temporal:
  temporal server start-dev

# Run the server
[no-cd]
@worker:
  cargo run --bin main

# Run the client
[no-cd]
@client:
  cargo run --bin client
