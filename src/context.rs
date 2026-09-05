use deadpool_postgres::Pool;
use reqwest::Client;
use tonic::service::interceptor::InterceptedService;
use tonic::transport::Channel;

use crate::interceptor::AuthInterceptor;
use crate::processor::config::InstitutionConfig;

/// Every backend gRPC client (`engine`, `crudgrpc`, `uidgen`) requires an
/// `authorization` header on every call - these aliases keep that
/// requirement visible in every function signature that holds one of
/// these clients, rather than the bare `Channel` type that used to make
/// it easy to construct one without an interceptor at all.
pub type EngineClient =
    crate::engine::engine_service_client::EngineServiceClient<InterceptedService<Channel, AuthInterceptor>>;
pub type CrudClient =
    crate::crudgrpc::crud_service_client::CrudServiceClient<InterceptedService<Channel, AuthInterceptor>>;
pub type UidgenClient =
    crate::uidgen::uid_gen_service_client::UidGenServiceClient<InterceptedService<Channel, AuthInterceptor>>;
#[derive(Clone)]
pub struct ProcessingContext {
    pub db_pool: Pool,
    pub http_client: Client,
    pub engine_client: EngineClient,
    pub crud_client: CrudClient,
    pub uidgen_client: UidgenClient,
    pub config: InstitutionConfig,
}
