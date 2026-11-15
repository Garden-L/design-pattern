# 03 — Plugin system

Register different concrete implementations under names and execute them
through one object-safe interface. Adapt existing stores and a legacy reader,
cache successful store reads, and preserve typed errors and their sources.
Prerequisites: [chapter 02](../../02-structural/README.md) and
[chapter 04](../../04-rust-native/README.md).

From the workspace root:

```sh
cargo +stable run --quiet -p plugin-system
cargo +stable test -p plugin-system
```

The demo prints:

```text
Rust guide
Rust guide
loads=1
```

| Requirement | Pattern / original chapter | Project boundary |
| --- | --- | --- |
| Translate an existing API | [Adapter, chapter 02](../../02-structural/src/adapter.rs) | DocumentPlugin adapts DocumentStore; LegacyAdapter adapts LegacyReader |
| Skip repeated successful reads | [Proxy, chapter 02](../../02-structural/src/proxy.rs) | DocumentPlugin owns the actual CachedProxy from that chapter |
| Store heterogeneous implementations | [Boxed products, chapter 01](../../01-creational/src/factory_method.rs), [heterogeneous nodes, chapter 02](../../02-structural/src/composite.rs) | PluginRegistry stores Box<dyn Plugin>, making type erasure explicit |
| Preserve causes | [Error design, chapter 04](../../04-rust-native/src/error_design.rs) | PluginError::App → AppError::Io → io::Error via source() |

[src/lib.rs](src/lib.rs) contains the object-safe `Plugin::execute(&self, key)`
interface, adapters, registry, and errors. Registering a blank name or duplicate
fails without replacing the original. Unknown names and blank document keys
return typed errors. Keys/names are otherwise exact, case-sensitive strings.
Store failures remain `PluginError::Store(StoreError)`; the upstream StoreError
does not implement Error, so it is retained as a variant rather than as a
`source()`. Legacy I/O failures preserve the complete error source chain.

[Tests](tests/plugins.rs) inject a fake Plugin through the public interface,
dispatch heterogeneous implementations, prove a cache hit skips the store,
prove failures are retried, downcast the I/O cause, and reject invalid inputs
before store access.

Plugins are compiled Rust implementations chosen at runtime. This is not a
dynamic-library loader or a stable binary ABI. The chapter proxy is
single-threaded with no expiry or invalidation; cache entries can grow and
become stale. LegacyAdapter does not add caching. For a closed implementation
set, prefer an enum; for one implementation, use the concrete type directly.
Exercise: add a third plugin and test its own success and error paths without
changing the registry. See the
[project lesson](../../../docs/course/06-projects.md#plugin-system).
