#!/usr/bin/env just --justfile

set dotenv-load

path := invocation_directory()

alias c:=client
alias s:=worker
alias f:=fmt

default:
  @echo {{path}}

# Cargo fmt
[group('dev')]
@fmt:
  cargo fmt

# Run a dev temporal server
[group('run')]
@temporal:
  temporal server start-dev

# Run the server
[group('run')]
[no-cd]
@worker:
  cargo run --bin main

# Run the client
[group('run')]
[no-cd]
@client crate='client':
  cargo run --bin {{crate}}

# Generate a new project using hello-world structure.
[group('dev')]
@gen project-name project-version='0.1.0':
  cargo new {{project-name}}
  rm {{project-name}}/src/main.rs
  cargo generate -o --init --path {{justfile_directory()}}/.template --name {{project-name}} --destination {{project-name}} --define project_version={{project-version}}
