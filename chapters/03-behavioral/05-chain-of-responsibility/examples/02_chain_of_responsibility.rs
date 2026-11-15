use chain_of_responsibility::{EventManager, ChainError, Handler, Request, Response};

struct Authorization;
impl Handler for Authorization {
    fn handle(&self, request: &Request) -> Result<Response, ChainError> {
        if request.0 == "private" {
            Err(ChainError::Rejected)
        } else {
            Err(ChainError::Continue)
        }
    }
}

struct Lookup;
impl Handler for Lookup {
    fn handle(&self, request: &Request) -> Result<Response, ChainError> {
        if request.0 == "public" {
            Ok(Response("public data".into()))
        } else {
            Err(ChainError::Continue)
        }
    }
}

pub fn run(key: &str) -> Result<String, ChainError> {
    let mut chain = EventManager::new();
    chain.push(Authorization);
    chain.push(Lookup);
    chain
        .handle(&Request(key.into()))
        .map(|response| response.0)
}
#[allow(dead_code)]
pub fn main() {}
