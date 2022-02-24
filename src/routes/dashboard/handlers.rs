use crate::routes::dashboard::models::DashboardUiComponent;
use crate::routes::safe_apps::handlers::safe_apps;
use crate::routes::safes::handlers::safes::get_safe_info_ex;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use tokio::join;

pub async fn get_dashboard(
    context: &RequestContext,
    chain_id: &String,
    safe_address: &String,
) -> ApiResult<Vec<DashboardUiComponent>> {
    let (safe_state, safe_apps) = join!(
        get_safe_info_ex(context, chain_id, safe_address),
        safe_apps(context, chain_id, &None)
    );
    // our approach is defensive if a single component errors, we still return those which don't
    Ok(vec![
        safe_state.map_or(DashboardUiComponent::ErrorLoadingComponent, |safe_state| {
            DashboardUiComponent::Safe(safe_state)
        }),
        safe_apps.map_or(DashboardUiComponent::ErrorLoadingComponent, |safe_apps| {
            DashboardUiComponent::SafeApps(safe_apps)
        }),
        DashboardUiComponent::LatestExecutedTxs,
    ])
}
