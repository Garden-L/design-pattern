# 01 — Notification CLI

Compose the existing chapter APIs into a command: validate a report, choose a
notification product and pricing rule, and render a receipt. Prerequisites:
[creation](../../01-creational/README.md),
[structure](../../02-structural/README.md), and
[behavior](../../03-behavioral/README.md).

From the workspace root, on stable Rust:

```sh
cargo +stable run --quiet -p notification-cli -- --channel email --title Release --message Ready --pricing discount
cargo +stable run --quiet -p notification-cli -- --help
cargo +stable test -p notification-cli
```

The first command prints:

```text
EMAIL: Release
Ready
Price: 90
```

`--channel email|sms`, `--title TEXT`, and `--message TEXT` are required.
`--pricing regular|discount` defaults to regular (100 teaching units); discount
is 90. Values are separate arguments, may contain Unicode, and cannot begin
with `--`. Quote text containing spaces. `--help` is a standalone command.
Unknown/duplicate options, missing values, unsupported selections, and blank
title/body produce stderr and exit 2, with no delivery on stdout. Success
exits 0. These factories format local strings; they do not contact a provider.

| Requirement | Pattern / original chapter API | Project boundary |
| --- | --- | --- |
| Select email or SMS | [Factory, chapter 01](../../01-creational/src/factory_method.rs) | CLI selects a borrowed factory; factory owns its product |
| Require title and body | [Builder, chapter 01](../../01-creational/src/builder.rs) | ReportBuilder validates before delivery |
| Substitute pricing | [Strategy, chapter 03](../../03-behavioral/src/strategy.rs) | Regular or TenPercentOff through PricingStrategy |
| Coordinate a single send | [Facade, chapter 02](../../02-structural/src/facade.rs) | Local NotificationFacade composes these notification APIs |

[src/lib.rs](src/lib.rs) owns argument parsing and the facade;
[src/main.rs](src/main.rs) owns process I/O and exit status.
[Command tests](tests/commands.rs) invoke the actual executable and check
receipts, Unicode, help, validation, and process failure behavior.

The order facade teaches orchestration but has inventory/payment invariants;
it is not repurposed here. No chapter API changes are needed. For one fixed
channel and rule, a function and enum are simpler than extensible traits.
Exercise: inject a third factory through `NotificationFacade::send`, test its
receipt, then add an explicit CLI selection. See the
[project lesson](../../../docs/course/06-projects.md#notification-cli).
