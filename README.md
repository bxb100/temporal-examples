# Temporal Examples

This repository want to rewrite all examples
from [temporalio/samples-typescript](https://github.com/temporalio/samples-typescript)

## Examples

| Example                                                                        | According                                                                                                             | Extra                                                                                                                                                            |
|--------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [hello-world](./hello-world)                                                   | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/hello-world)                          |                                                                                                                                                                  |
| [activities-cancellation-heartbeating](./activities-cancellation-heartbeating) | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/activities-cancellation-heartbeating) | [temporal-samples-rust](https://github.com/cosm-public/temporal-samples-rust/tree/45eb692928195c1cd3325942277792f21ab86715/activities-cancellation-heartbeating) |
| [activities-dependency-injection](./activities-dependency-injection)           | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/activities-dependency-injection)      | todo                                                                                                                                                             |
| [activities-examples](./activities-examples)                                   | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/activities-examples)                  | [temporal-samples-rust](https://github.com/cosm-public/temporal-samples-rust/tree/45eb692928195c1cd3325942277792f21ab86715/activities-examples)                  |
| [child-workflows](./child-workflows)                                           | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/child-workflows)                      |                                                                                                                                                                  |
| [continue-as-new](./continue-as-new)                                           | [typescript version](https://github.com/temporalio/samples-typescript/tree/main/continue-as-new)                      |                                                                                                                                                                  |

## References

- https://github.com/cosm-public/temporal-samples-rust
- https://github.com/temporalio/sdk-core

## TODO

- Using [inventory](https://github.com/dtolnay/inventory) to mock threadLocal activity context
  inject
- Using proc-macros to remove boilerplate code
- figure out workflow FSM poll trigger execution non-static code multiple time, and the call path
