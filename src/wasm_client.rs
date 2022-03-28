use anyhow;
use prost_types;

#[cfg(feature = "tls")]
use tonic::transport::channel::ClientTlsConfig;
#[cfg(feature = "tls")]

use tonic::{metadata::MetadataValue};
use tonic::{Request};
use tonic::{service::interceptor::InterceptedService, Status};

use super::inference::grpc_inference_service_client::GrpcInferenceServiceClient;

type InterceptorFn = Box<dyn Fn(Request<()>) -> Result<Request<()>, Status> + Send>;

pub struct Client {
    pub client: GrpcInferenceServiceClient<grpc_web_client::Client>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Create dataref: `dataref` unexpectedly missing in server response")]
    DataRefMissing {},
    #[error("No OctoML access token is set in the configuration files.")]
    NoAccessToken {},
}

impl Client {
    pub fn new(url: String) -> anyhow::Result<Self> {
        //     let channel = Channel::from_shared(url)?
        //     .connect()
        //     .await?;

        // // let token = MetadataValue::from_shared(format!("Bearer {}", access_token).into())?;

        // let interceptor_fn = Box::new(move |mut req: Request<()>| {
        //     // req.metadata_mut().insert("authorization", token.clone());
        //     Ok(req)
        // }) as InterceptorFn;

        // let client = GrpcInferenceServiceClient::with_interceptor(channel, interceptor_fn);
        let client = grpc_web_client::Client::new(url);
        let client = GrpcInferenceServiceClient::new(client);

        Ok(Client { client })
    }
}
