use super::check_token;
use crate::{error::Error, resources};
use actix_files::NamedFile;
use actix_web::{web::Path, HttpRequest};

/// Endpoint for retrieving files
pub async fn get_file_endpoint(
    path: Path<(String, String)>,
    req: HttpRequest,
) -> Result<NamedFile, Error> {
    let res_name = &path.0;
    let file_name = &path.1;

    let resources = resources::get();
    let resource = resources.get(&res_name).ok_or(Error::NotFound)?;

    if !check_token(&req, &resource) {
        return Err(Error::Unauthorized);
    }

    let rfile = resource.get_file(&file_name)?;

    Ok(NamedFile::open(rfile.path())?)
}
