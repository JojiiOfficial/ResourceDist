use super::check_token;
use crate::{error::Error, resources};
use actix_web::{
    web::{block, Path},
    HttpRequest,
};

/// Endpoind for getting the hash of a given file in a configured resource
pub async fn get_hash_endpoint(
    path: Path<(String, String)>,
    req: HttpRequest,
) -> Result<String, Error> {
    let res_name = &path.0;
    let file_name = &path.1;

    let resources = resources::get();
    let resource = resources.get(&res_name).ok_or(Error::NotFound)?;

    if !check_token(&req, &resource) {
        return Err(Error::Unauthorized);
    }

    let file = resource.get_file(&file_name)?;

    let hash = block(move || file.hash()).await??;

    Ok(hash)
}
