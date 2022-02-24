use super::handlers;
use crate::cache::cache_operations::CacheResponse;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get(
    "/v1/chains/<chain_id>/safes/<safe_address>/dashboard?<fiat>&<trusted_tokens>&<exclude_spam_tokens>"
)]
pub async fn get_dashboard(
    context: RequestContext,
    chain_id: String,
    safe_address: String,
    fiat: Option<String>,
    trusted_tokens: Option<bool>,
    exclude_spam_tokens: Option<bool>,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(&context)
        .resp_generator(|| {
            handlers::get_dashboard(
                &context,
                &chain_id,
                &safe_address,
                &fiat,
                &trusted_tokens,
                &exclude_spam_tokens,
            )
        })
        .execute()
        .await
}
