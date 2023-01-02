use crate::{error::Error, resources};
use actix_web::web::{block, Path};

/// Endpoind for getting the hash of a given file in a configured resource
pub async fn get_hash_endpoint(path: Path<(String, String)>) -> Result<String, Error> {
    let res_name = &path.0;
    let file_name = &path.1;

    let file = resources::get()
        .get(&res_name)
        .ok_or(Error::NotFound)?
        .get_file(&file_name)?;

    let hash = block(move || file.hash()).await??;

    Ok(hash)
}
