#!/usr/bin/env just --justfile

set dotenv-load

path := invocation_directory()

alias c:=client
alias s:=worker
alias f:=fmt

default:
  @echo {{path}}

# cargo fmt
[group('dev')]
@fmt:
  cargo fmt --check

# run the server
[group('run')]
[no-cd]
@worker:
  cargo run --bin main

# run the client
[group('run')]
[no-cd]
@client crate='client':
  cargo run --bin {{crate}}

# generate a new project using hello-world structure
[group('dev')]
@gen project-name project-version='0.1.0':
  cargo new {{project-name}}
  rm {{project-name}}/src/main.rs
  cargo generate -o --init --path {{justfile_directory()}}/.template --name {{project-name}} --destination {{project-name}} --define project_version={{project-version}}

# run a dev temporal server
[group('dev-server-cli')]
@temporal:
  temporal server start-dev --ui-port 8080

# up the docker compose with postgresql and elasticsearch
[group('dev-server-docker')]
up:
  #!/usr/bin/env sh
  cd {{source_directory()}}/docker-compose
  docker compose -f docker-compose.yml up -d
  open -u http://localhost:8080

# down the docker compose
[group('dev-server-docker')]
down:
  #!/usr/bin/env sh
  cd {{source_directory()}}/docker-compose
  docker compose -f docker-compose.yml down -v

# stop the docker compose
[group('dev-server-docker')]
stop:
  #!/usr/bin/env sh
  cd {{source_directory()}}/docker-compose
  docker compose -f docker-compose.yml stop
