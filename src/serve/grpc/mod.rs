use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::Receiver;
use tonic::transport::{Certificate, Server, ServerTlsConfig};

use utxorpc::proto::sync::v1::chain_sync_service_server::ChainSyncServiceServer;

use crate::prelude::*;
use crate::storage::rolldb::RollDB;

mod sync;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    listen_address: String,
    tls_client_ca_root: Option<PathBuf>,
}

pub async fn serve(
    config: Config,
    db: RollDB,
    sync_events: Receiver<gasket::messaging::Message<RollEvent>>,
) -> Result<(), Error> {
    let addr = config.listen_address.parse().unwrap();
    let service = sync::ChainSyncServiceImpl::new(db, sync_events);
    let service = ChainSyncServiceServer::new(service);

    let mut server = Server::builder().accept_http1(true);

    if let Some(pem) = config.tls_client_ca_root {
        let pem = std::env::current_dir().unwrap().join(pem);
        let pem = std::fs::read_to_string(pem).map_err(Error::config)?;
        let pem = Certificate::from_pem(pem);

        let tls = ServerTlsConfig::new().client_ca_root(pem);

        server = server.tls_config(tls).map_err(Error::config)?;
    }

    server
        // GrpcWeb is over http1 so we must enable it.
        .add_service(tonic_web::enable(service))
        .serve(addr)
        .await
        .map_err(Error::server)?;

    Ok(())
}
