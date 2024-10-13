use tonic_reflection::server::{ServerReflection, ServerReflectionServer};
use zoeaubert_proto::FILE_DESCRIPTOR_SET;

pub mod utils;
pub mod cache;

pub mod zoeaubert_proto {
    pub mod webserver {
        tonic::include_proto!("me.zoeaubert.webserver");
    }
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("zoeaubert_descriptor");
}

pub fn make_reflection_descriptor_service(
) -> Result<ServerReflectionServer<impl ServerReflection>, tonic_reflection::server::Error> {
    tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
}
