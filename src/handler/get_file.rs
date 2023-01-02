use crate::{error::Error, resources};
use actix_files::NamedFile;
use actix_web::web::Path;

/// Endpoint for retrieving files
pub async fn get_file_endpoint(path: Path<(String, String)>) -> Result<NamedFile, Error> {
    let res_name = &path.0;
    let file_name = &path.1;

    let rfile = resources::get()
        .get(&res_name)
        .ok_or(Error::NotFound)?
        .get_file(&file_name)?;

    Ok(NamedFile::open(rfile.path())?)
}
