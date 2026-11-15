//! Required request fields encoded in the builder's type.
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct MissingMethod;
#[derive(Debug, Clone, Copy)]
pub struct MissingUrl;
#[derive(Debug, Clone, Copy)]
pub struct Ready;

#[derive(Debug, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub url: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BuildError {
    MissingMethod,
    MissingUrl,
}

pub struct RequestBuilder<M = Ready, U = Ready> {
    method: Option<String>,
    url: Option<String>,
    state: PhantomData<(M, U)>,
}

impl RequestBuilder<MissingMethod, MissingUrl> {
    pub fn new() -> Self {
        Self {
            method: None,
            url: None,
            state: PhantomData,
        }
    }
}
impl Default for RequestBuilder<MissingMethod, MissingUrl> {
    fn default() -> Self {
        Self::new()
    }
}
impl<U> RequestBuilder<MissingMethod, U> {
    pub fn method(self, value: impl Into<String>) -> RequestBuilder<Ready, U> {
        RequestBuilder {
            method: Some(value.into()),
            url: self.url,
            state: PhantomData,
        }
    }
}
impl<M> RequestBuilder<M, MissingUrl> {
    pub fn url(self, value: impl Into<String>) -> RequestBuilder<M, Ready> {
        RequestBuilder {
            method: self.method,
            url: Some(value.into()),
            state: PhantomData,
        }
    }
}
impl RequestBuilder<Ready> {
    pub fn build(self) -> Request {
        Request {
            method: self.method.expect("Ready method"),
            url: self.url.expect("Ready URL"),
        }
    }
}

/// A missing field cannot be built.
///
/// ```compile_fail
/// use rust_native::typestate::RequestBuilder;
/// let _ = RequestBuilder::new().method("GET").build();
/// ```
///
/// ```compile_fail
/// use rust_native::typestate::RequestBuilder;
/// let _ = RequestBuilder::new().url("/health").build();
/// ```
///
/// ```compile_fail
/// use rust_native::typestate::RequestBuilder;
/// let _ = RequestBuilder::new().method("GET").method("POST");
/// ```
pub fn compile_fail_examples() {}
