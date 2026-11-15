# Exercise 1: Add a product

Add a `PushFactory` and a notification product to `src/factory_method.rs`.
`PushFactory.create("Ready").deliver()` must return `PUSH: Ready`.
Keep the `NotificationFactory::create` signature. Add the corresponding
simple, duplicated `before/factory_method.rs` function and extend the shared
behavior test. Discuss whether an enum would now be simpler than a trait.
