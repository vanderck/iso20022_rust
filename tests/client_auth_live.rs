//! Live verification that `engine_client`/`crud_client`/`uidgen_client` actually
//! attach the `authorization` header on every outgoing call, against real
//! `engine` and `uidgen` server processes (both enforce it; `crudgrpc` does
//! not, so it isn't exercised here).
//!
//! Ignored by default — requires `engine` and `uidgen` running locally with
//! `api_key = "test-api-key"` (see each repo's `test_config.json`):
//!
//! ```ignore
//! cargo test --test client_auth_live -- --ignored --nocapture
//! ```

use iso20022::interceptor::AuthInterceptor;
use tonic::transport::Channel;

const ENGINE_ADDR: &str = "http://127.0.0.1:50052";
const UIDGEN_ADDR: &str = "http://127.0.0.1:50060";
const CORRECT_KEY: &str = "test-api-key";
const WRONG_KEY: &str = "not-the-right-key";

#[tokio::test]
#[ignore]
async fn uidgen_client_with_correct_key_succeeds() {
    let channel = Channel::from_shared(UIDGEN_ADDR).unwrap().connect().await.unwrap();
    let mut client = iso20022::uidgen::uid_gen_service_client::UidGenServiceClient::with_interceptor(
        channel,
        AuthInterceptor::new(CORRECT_KEY).unwrap(),
    );
    let resp = client.next_id(iso20022::uidgen::NextIdRequest {}).await;
    assert!(resp.is_ok(), "expected next_id to succeed with correct key, got {:?}", resp);
    assert!(resp.unwrap().into_inner().id > 0);
}

#[tokio::test]
#[ignore]
async fn uidgen_client_with_wrong_key_is_rejected() {
    let channel = Channel::from_shared(UIDGEN_ADDR).unwrap().connect().await.unwrap();
    let mut client = iso20022::uidgen::uid_gen_service_client::UidGenServiceClient::with_interceptor(
        channel,
        AuthInterceptor::new(WRONG_KEY).unwrap(),
    );
    let resp = client.next_id(iso20022::uidgen::NextIdRequest {}).await;
    let err = resp.expect_err("expected next_id to be rejected with a wrong key");
    assert_eq!(err.code(), tonic::Code::Unauthenticated, "got: {err:?}");
}

#[tokio::test]
#[ignore]
async fn uidgen_client_with_no_interceptor_is_rejected() {
    // Reproduces the pre-fix state: a client built with `::new` sends no
    // `authorization` header at all and must be rejected the same way.
    let channel = Channel::from_shared(UIDGEN_ADDR).unwrap().connect().await.unwrap();
    let mut client = iso20022::uidgen::uid_gen_service_client::UidGenServiceClient::new(channel);
    let resp = client.next_id(iso20022::uidgen::NextIdRequest {}).await;
    let err = resp.expect_err("expected next_id to be rejected with no auth header");
    assert_eq!(err.code(), tonic::Code::Unauthenticated, "got: {err:?}");
}

#[tokio::test]
#[ignore]
async fn engine_client_with_correct_key_reaches_business_logic() {
    let channel = Channel::from_shared(ENGINE_ADDR).unwrap().connect().await.unwrap();
    let mut client = iso20022::engine::engine_service_client::EngineServiceClient::with_interceptor(
        channel,
        AuthInterceptor::new(CORRECT_KEY).unwrap(),
    );
    let resp = client
        .authorisation(iso20022::engine::Authorisation {
            benefactor: -1,
            beneficiary: -2,
            amount: "10.00".into(),
            currency: "EUR".into(),
            ..Default::default()
        })
        .await;
    // Nonexistent benefactor/beneficiary ids are a *business-logic* error,
    // not Unauthenticated — reaching that error at all proves the
    // authorization header was accepted.
    let err = resp.expect_err("expected a business-logic error for a bogus request");
    assert_ne!(err.code(), tonic::Code::Unauthenticated, "got: {err:?}");
}

#[tokio::test]
#[ignore]
async fn engine_client_with_wrong_key_is_rejected() {
    let channel = Channel::from_shared(ENGINE_ADDR).unwrap().connect().await.unwrap();
    let mut client = iso20022::engine::engine_service_client::EngineServiceClient::with_interceptor(
        channel,
        AuthInterceptor::new(WRONG_KEY).unwrap(),
    );
    let resp = client
        .authorisation(iso20022::engine::Authorisation {
            benefactor: -1,
            beneficiary: -2,
            amount: "10.00".into(),
            currency: "EUR".into(),
            ..Default::default()
        })
        .await;
    let err = resp.expect_err("expected authorisation to be rejected with a wrong key");
    assert_eq!(err.code(), tonic::Code::Unauthenticated, "got: {err:?}");
}
