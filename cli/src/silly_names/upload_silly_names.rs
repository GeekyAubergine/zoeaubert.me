use shared::zoeaubert_proto::webserver::{SillyName, UpdateSillyNamesRequest};
use tonic::Request;
use tracing::info;

use crate::{error::TonicError, prelude::Result, utils::api_client::ApiClient};

use super::read_silly_names_file;

pub async fn upload_silly_names(api_client: &ApiClient) -> Result<()> {
    info!("Uploading silly names");
    let silly_names = read_silly_names_file().await?;

    let silly_names = silly_names
        .iter()
        .map(|r| SillyName {
            name: r.name.clone(),
            creator: r.creator.clone(),
        })
        .collect::<Vec<SillyName>>();

    let request = Request::new(UpdateSillyNamesRequest { names: silly_names });

    api_client
        .silly_names_client()
        .update_silly_names(request)
        .await
        .map_err(TonicError::server_returned_status)?;

    Ok(())
}
