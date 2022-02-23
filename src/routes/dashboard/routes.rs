use crate::utils::{context::RequestContext, errors::ApiResult};
use rocket::response::content;
use crate::cache::cache_operations::CacheResponse;
use super::handlers;

#[get("/v1/chains/<chain_id>/safes/<safe_address>/dashboard")]
pub async fn get_dashboard(
    context: RequestContext,
    chain_id: String,
    safe_address:String,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(&context)
        .resp_generator(|| handlers::get_dashboard(&context, &chain_id, &safe_address))
        .execute()
        .await
}
