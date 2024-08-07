use shared::zoeaubert_proto::webserver::{
    silly_names_server::SillyNames, UpdateSillyNamesRequest, UpdateSillyNamesResponse,
};
use tonic::{Request, Response, Status};

use crate::{
    application::commands::update_silly_names_command::update_silly_names_command,
    infrastructure::app_state::AppState,
};

#[derive(Debug, Clone)]
pub struct SillyNamesGprcService {
    state: AppState,
}

impl SillyNamesGprcService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl SillyNames for SillyNamesGprcService {
    async fn update_silly_names(
        &self,
        request: Request<UpdateSillyNamesRequest>,
    ) -> Result<Response<UpdateSillyNamesResponse>, Status> {
        let headers = request.metadata().get("Authorization");
        let silly_names = request.into_inner().names;

        let silly_names = silly_names
            .iter()
            .map(|name| name.name.clone())
            .collect::<Vec<String>>();

        update_silly_names_command(&self.state, &silly_names)
            .await
            .map_err(|e| e.into_tonic_status())?;

        Ok(Response::new(UpdateSillyNamesResponse {}))
    }
}
