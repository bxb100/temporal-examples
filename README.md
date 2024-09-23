# Temporal Examples

This repository want to rewrite all examples
from [temporalio/samples-typescript](https://github.com/temporalio/samples-typescript)

## Examples

| Example                                                                           | According                                                                                                             | Extra                                    |
|-----------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|------------------------------------------|
| [hello-world](./hello-world)                                                      | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/hello-world)                          |                                          |
| [activities-cancellation-heartbeating 🦀](./activities-cancellation-heartbeating) | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/activities-cancellation-heartbeating) |                                          |
| [activities-dependency-injection](./activities-dependency-injection)              | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/activities-dependency-injection)      | todo                                     |
| [activities-examples 🦀](./activities-examples)                                   | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/activities-examples)                  |                                          |
| [child-workflows](./child-workflows)                                              | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/child-workflows)                      |                                          |
| [continue-as-new](./continue-as-new)                                              | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/continue-as-new)                      |                                          |
| [~~cron-workflows~~ 🚫](./cron-workflows)                                         | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/cron-workflows)                       |                                          |
| [schedules 🦀](./schedules)                                                       | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/schedules)                            |                                          |
| [~~scratchpad~~ ☁️](./scratchpad)                                                 | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/scratchpad)                           |                                          |
| [search-attributes](./search-attributes)                                          | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/search-attributes)                    | `just up` <br/> `just add`               |
| [custom-logger](./custom-logger)                                                  | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/custom-logger)                        | `just s2`                                |
| [dsl-interpreter](./dsl-interpreter)                                              | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/dsl-interpreter)                      | `just workflow1` <br /> `just workflow2` |

> 🦀 : means the example originally
> from [temporal-samples-rust](https://github.com/cosm-public/temporal-samples-rust)
>
> ☁️ : means the example not necessary to RIIR
>
> 🚫 : means the example deprecated in typescript version

## References

- https://github.com/cosm-public/temporal-samples-rust
- https://github.com/temporalio/sdk-core

## TODO

- Using [inventory](https://github.com/dtolnay/inventory) to mock threadLocal activity context
  inject
- Using proc-macros to remove boilerplate code
- figure out workflow FSM poll trigger execution non-static code multiple time, and the call path
