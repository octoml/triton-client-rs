use anyhow;
use prost_types;

use tonic::metadata::MetadataValue;
use tonic::transport::channel::ClientTlsConfig;
use tonic::{service::interceptor::InterceptedService, Status};
use tonic::{transport::Channel, Request};

use super::inference::grpc_inference_service_client::GrpcInferenceServiceClient;

type InterceptorFn = Box<dyn Fn(Request<()>) -> Result<Request<()>, Status> + Send>;

pub struct Client {
    pub client: GrpcInferenceServiceClient<InterceptedService<Channel, InterceptorFn>>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Create dataref: `dataref` unexpectedly missing in server response")]
    DataRefMissing {},
    #[error("No OctoML access token is set in the configuration files.")]
    NoAccessToken {},
}

impl Client {
    pub async fn new(url: String, access_token: Option<String>) -> anyhow::Result<Self> {
        let mut channel = Channel::from_shared(url)?;

        if access_token.is_some() {
            channel = channel
            .tls_config(ClientTlsConfig::new())?;
        }

        let channel = channel.connect().await?;

        let interceptor_fn = Box::new(move |mut req: Request<()>| {
            if let Some(access_token) = access_token.clone() {
                let token_header = MetadataValue::from_shared(format!("Bearer {}", access_token).into())?;
                req.metadata_mut().insert("authorization", token_header.clone());
            }

            Ok(req)
        }) as InterceptorFn;

        let client = GrpcInferenceServiceClient::with_interceptor(channel, interceptor_fn);

        Ok(Client { client })
    }
}
