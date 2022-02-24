use crate::routes::collectibles::handlers::collectibles;
use crate::routes::dashboard::models::DashboardUiComponent;
use crate::routes::safe_apps::handlers::safe_apps;
use crate::routes::safes::handlers::safes::get_safe_info_ex;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use serde_json::value::RawValue;
use serde_json::Error;
use tokio::join;

pub async fn get_dashboard(
    context: &RequestContext,
    chain_id: &String,
    safe_address: &String,
) -> ApiResult<Vec<DashboardUiComponent>> {
    let (safe_state, safe_apps, collectibles) = join!(
        get_safe_info_ex(context, chain_id, safe_address),
        safe_apps(context, chain_id, &None),
        collectibles(context, chain_id, safe_address, None, None)
    );
    // our approach is defensive if a single component errors, we still return those which don't
    Ok(vec![
        safe_state.map_or(DashboardUiComponent::ErrorLoadingComponent, |safe_state| {
            DashboardUiComponent::Safe(safe_state)
        }),
        safe_apps.map_or(DashboardUiComponent::ErrorLoadingComponent, |safe_apps| {
            DashboardUiComponent::SafeApps(safe_apps)
        }),
        collectibles.map_or(
            DashboardUiComponent::ErrorLoadingComponent,
            |collectibles| match RawValue::from_string(collectibles.0) {
                Ok(json) => DashboardUiComponent::Collectibles(json),
                Err(_) => DashboardUiComponent::ErrorLoadingComponent,
            },
        ),
        DashboardUiComponent::LatestExecutedTxs,
    ])
}
