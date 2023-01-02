pub mod get_file;
pub mod get_hash;
pub mod healthy;
pub mod ready;

use crate::resources::Resource;
use actix_web::HttpRequest;

pub(crate) fn check_token(req: &HttpRequest, res: &Resource) -> bool {
    let res_token = res.configuration().access_token.trim();

    // We don't need token for empty configs
    if res_token.is_empty() {
        return true;
    }

    let token = match get_token(req) {
        Some(t) => t,
        None => return false,
    };

    token == res_token
}

pub(crate) fn get_token(req: &HttpRequest) -> Option<String> {
    let auth_header = req.headers().get("Authorization")?.to_str().ok()?;
    if !auth_header.contains("Bearer") {
        return None;
    }
    Some(auth_header.trim().split(' ').rev().next()?.to_string())
}
