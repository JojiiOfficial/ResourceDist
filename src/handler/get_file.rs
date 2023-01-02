use crate::error::Error;
use actix_web::web::Path;

/// Endpoint for retrieving files
pub async fn get_file_endpoint(path: Path<(String, String)>) -> Result<String, Error> {
    todo!()
}
