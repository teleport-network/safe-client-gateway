use crate::utils::{context::RequestContext, errors::ApiResult};
use rocket::response::content;

#[get("/v1/chains/<chain_id>/dashboard")]
pub async fn get_dashboard(
    context: RequestContext,
    chain_id: String,
) -> ApiResult<content::Json<String>> {
    Ok(content::Json(String::from("fancy dashboard")))
}
