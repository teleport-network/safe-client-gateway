use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::dashboard::models::DashboardUiComponent;
use crate::routes::safe_apps::handlers::safe_apps;
use crate::routes::safes::handlers::safes::get_safe_info_ex;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use tokio::try_join;

pub async fn get_dashboard(
    context: &RequestContext,
    chain_id: &String,
    safe_address: &String,
) -> ApiResult<Vec<DashboardUiComponent>> {
    let info_provider = DefaultInfoProvider::new(chain_id, context);

    let (safe_state, safe_apps) = try_join!(
        get_safe_info_ex(context, chain_id, safe_address),
        safe_apps(context, chain_id, &None)
    )?;
    Ok(vec![
        DashboardUiComponent::Safe(safe_state),
        DashboardUiComponent::SafeApps(safe_apps),
        DashboardUiComponent::LatestExecutedTxs,
    ])
}
