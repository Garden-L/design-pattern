use behavioral::chain::{Chain, ChainError, Handler, Request, Response};

struct Authorization;
impl Handler for Authorization {
    fn handle(&self, request: &Request) -> Result<Response, ChainError> {
        if request.0 == "private" {
            Err(ChainError::Rejected)
        } else {
            Err(ChainError::Pass)
        }
    }
}

struct Lookup;
impl Handler for Lookup {
    fn handle(&self, request: &Request) -> Result<Response, ChainError> {
        if request.0 == "public" {
            Ok(Response("public data".into()))
        } else {
            Err(ChainError::Pass)
        }
    }
}

pub fn run(key: &str) -> Result<String, ChainError> {
    let mut chain = Chain::new();
    chain.push(Authorization);
    chain.push(Lookup);
    chain
        .handle(&Request(key.into()))
        .map(|response| response.0)
}
