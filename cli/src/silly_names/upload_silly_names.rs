use shared::api_definitions::silly_names::UpdateSillyNamesRequest;
use tracing::info;

use crate::{
    api_client::{ApiClient, ApiResponse},
    prelude::Result,
};

use super::read_silly_names_file;

const URL_PATH: &str = "api/v1/silly-names";

pub async fn upload_silly_names(api_client: &ApiClient) -> Result<()> {
    info!("Uploading silly names");
    let silly_names = read_silly_names_file().await?;

    let silly_names = silly_names
        .iter()
        .map(|r| r.name.to_owned())
        .collect::<Vec<String>>();

    let silly_names_request: UpdateSillyNamesRequest = UpdateSillyNamesRequest { silly_names };

    api_client.put(URL_PATH, &silly_names_request).await?;

    Ok(())
}
