use crate::utils::{context::RequestContext, errors::ApiResult};
use crate::routes::dashboard::models::DashboardUiComponent;

pub async fn get_dashboard(context: &RequestContext, chain_id: &str, safe_address: &str) -> ApiResult<Vec<DashboardUiComponent>>{
    Ok(vec![DashboardUiComponent::TokenBalances, DashboardUiComponent::LatestExecutedTxs])
}