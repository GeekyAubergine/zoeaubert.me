use shared::{
    utils::date::parse_date,
    zoeaubert_proto::webserver::{
        micro_posts_server::MicroPosts, UpdateMicroPostRequest, UpdateMicroPostResponse,
    },
};
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::{
    application::commands::microposts::create_or_update_micropost::create_or_update_micropost,
    domain::models::{
        media::{image::Image, Media},
        micro_post::{MicroPost, MicroPostUuid},
        tag::Tag,
    },
    infrastructure::{
        app_state::AppState,
        query_services::{
            micro_posts_query_service::commit_micropost, tags_query_service::commit_tags_for_entity,
        },
        services::{auth_service::authenticate_grpc, parse_uuid},
    },
};

#[derive(Debug, Clone)]
pub struct MicroPostsGrpcService {
    state: AppState,
}

impl MicroPostsGrpcService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl MicroPosts for MicroPostsGrpcService {
    async fn update_micro_post(
        &self,
        request: Request<UpdateMicroPostRequest>,
    ) -> Result<Response<UpdateMicroPostResponse>, Status> {
        authenticate_grpc(&request)?;

        let micro_post = request.into_inner().micro_post;

        let micro_post = match micro_post {
            Some(micro_post) => micro_post,
            None => return Err(Status::invalid_argument("MicroPost is required")),
        };

        let media: Vec<Media> = micro_post
            .images
            .iter()
            .map(|image| {
                Media::from_image(
                    Image::new(
                        &Uuid::parse_str(&image.uuid).unwrap(),
                        &image.url,
                        &image.alt,
                        image.width as u32,
                        image.height as u32,
                    )
                    .with_date(parse_date(&image.date).unwrap()),
                )
            })
            .collect();

        let tags = micro_post
            .tags
            .iter()
            .map(|tag| Tag::from_string(tag))
            .collect::<Vec<Tag>>();

        let uuid = parse_uuid(&micro_post.uuid).map_err(|e| e.into_tonic_status())?;

        create_or_update_micropost(
            MicroPostUuid::new(uuid),
            micro_post.slug,
            parse_date(&micro_post.date).unwrap(),
            micro_post.content,
            tags,
            media.iter().map(|media| media.uuid()).cloned().collect(),
            &self.state,
        )
        .await
        .map_err(|e| e.into_tonic_status())?;

        Ok(Response::new(UpdateMicroPostResponse {}))
    }
}
