## Generate new project

- `just gen {project name}`
- `just gen {project name} {project version}`

## Project structure

```text
.
├── Cargo.toml
└── src
    ├── activities
    │   ├── greet.rs
    │   └── mod.rs
    ├── bin
    │   ├── client.rs
    │   └── main.rs
    ├── lib.rs
    ├── worker.rs
    └── workflows.rs
```

## Run the project

> see more by using `just --list`

- `just s` to start the worker
- `just c` to start the client
- `just temporal` to start the temporal server
