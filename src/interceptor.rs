//! Shared client-side auth interceptor: attaches a fixed API key to every
//! outgoing gRPC request's `authorization` metadata. `engine`, `crudgrpc`,
//! and `uidgen` all require this on every call (constant-time compared
//! server-side), but previously none of `engine_client`, `crud_client`
//! sent any credential at all, so every call to either backend would be
//! rejected as `Unauthenticated` against a properly configured deployment
//! (this was the same gap `vanderck/bot` found and fixed in its own
//! `EngineClient`/`CrudClient` earlier).

use tonic::metadata::MetadataValue;
use tonic::service::Interceptor;
use tonic::{Request, Status};

#[derive(Clone)]
pub struct AuthInterceptor {
    token: MetadataValue<tonic::metadata::Ascii>,
}

impl AuthInterceptor {
    pub fn new(api_key: &str) -> Result<Self, tonic::metadata::errors::InvalidMetadataValue> {
        Ok(Self { token: MetadataValue::try_from(api_key)? })
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        req.metadata_mut().insert("authorization", self.token.clone());
        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_accepts_a_valid_ascii_key() {
        assert!(AuthInterceptor::new("test-api-key-123").is_ok());
    }

    #[test]
    fn new_accepts_an_empty_key() {
        // An empty string is still valid ASCII metadata - `MetadataValue`
        // doesn't reject it, even though an empty API key is obviously a
        // misconfiguration in practice (caught separately at config-load
        // time, not by this constructor).
        assert!(AuthInterceptor::new("").is_ok());
    }

    #[test]
    fn new_rejects_a_key_with_invalid_ascii_metadata_bytes() {
        // Metadata values must be visible ASCII; a raw newline is a classic
        // header-injection byte and must be rejected, not smuggled through.
        assert!(AuthInterceptor::new("key\nwith-newline").is_err());
    }

    #[test]
    fn new_rejects_a_key_with_a_control_character() {
        // Non-ASCII *visible* bytes (e.g. accented Latin-1 text) are actually
        // accepted as opaque "obs-text" by the underlying `http::HeaderValue`
        // - only genuine control bytes (other than tab) are rejected. A
        // stray control character is still a real malformed-key case worth
        // guarding against.
        assert!(AuthInterceptor::new("key-with-\x01-control-byte").is_err());
    }

    #[test]
    fn call_attaches_the_authorization_header() {
        let mut interceptor = AuthInterceptor::new("my-secret-key").expect("valid key");
        let req = Request::new(());
        let req = interceptor.call(req).expect("interceptor should not reject the request");
        let value = req.metadata().get("authorization").expect("authorization header must be set");
        assert_eq!(value.to_str().unwrap(), "my-secret-key");
    }

    #[test]
    fn call_overwrites_any_preexisting_authorization_header() {
        let mut interceptor = AuthInterceptor::new("the-real-key").expect("valid key");
        let mut req = Request::new(());
        req.metadata_mut().insert("authorization", MetadataValue::try_from("stale-or-forged").unwrap());
        let req = interceptor.call(req).expect("interceptor should not reject the request");
        assert_eq!(req.metadata().get("authorization").unwrap().to_str().unwrap(), "the-real-key");
    }
}
