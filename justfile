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
@client crate='client':
  cargo run --bin {{crate}}

@gen project-name project-version='0.1.0':
  @cargo new {{project-name}}
  @rm {{project-name}}/src/main.rs
  @cargo generate -o --init --path ./.template --name {{project-name}} --destination {{project-name}} --define project_version={{project-version}}
