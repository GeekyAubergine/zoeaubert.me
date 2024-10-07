use crate::{
    microblog_archive::upload_microblog_archive_posts::upload_microblob_archive_posts,
    microposts::upload_micro_posts::upload_micro_posts, prelude::Result,
    silly_names::upload_silly_names::upload_silly_names, utils::api_client::ApiClient,
};

pub async fn upload(api_client: &ApiClient) -> Result<()> {
    upload_silly_names(api_client).await?;
    upload_microblob_archive_posts(api_client).await?;
    upload_micro_posts(api_client).await?;

    Ok(())
}
