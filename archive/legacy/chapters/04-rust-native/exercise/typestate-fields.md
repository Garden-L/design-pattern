# Exercise: repair an incomplete request

Try to compile `RequestBuilder::new().method("POST").build()`. The doctest in
`src/typestate.rs` shows the expected compile failure. Add a URL before
`build`, then try setting the URL first and the method second. Both orders
must yield the same `Request`; an incomplete builder must still have no
`build` method. Explain why dynamic `Option<&str>` inputs still need a runtime
check at the API boundary.
