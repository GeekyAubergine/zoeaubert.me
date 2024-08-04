use crate::{api_client::ApiClient, prelude::Result, silly_names::upload_silly_names::upload_silly_names};

pub async fn upload(api_client: &ApiClient) -> Result<()> {
    upload_silly_names(api_client).await?;

    Ok(())
}
