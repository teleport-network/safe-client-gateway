use crate::routes::balances::handlers_v2::balances;
use crate::routes::collectibles::handlers::collectibles;
use crate::routes::dashboard::models::DashboardUiComponent;
use crate::routes::safe_apps::handlers::safe_apps;
use crate::routes::safes::handlers::safes::get_safe_info_ex;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use serde_json::value::RawValue;
use tokio::join;

pub async fn get_dashboard(
    context: &RequestContext,
    chain_id: &String,
    safe_address: &String,
    fiat: &Option<String>,
    trusted_tokens: &Option<bool>,
    exclude_spam_tokens: &Option<bool>,
) -> ApiResult<Vec<DashboardUiComponent>> {
    let fiat = fiat.as_deref().unwrap_or("EUR");
    let (safe_state, safe_apps, collectibles, balances) = join!(
        get_safe_info_ex(context, chain_id, safe_address),
        safe_apps(context, chain_id, &None),
        collectibles(
            context,
            chain_id,
            safe_address,
            trusted_tokens,
            exclude_spam_tokens
        ),
        balances(
            context,
            chain_id,
            safe_address,
            &fiat,
            trusted_tokens.unwrap_or(false),
            exclude_spam_tokens.unwrap_or(true)
        )
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
        balances.map_or(DashboardUiComponent::ErrorLoadingComponent, |balances| {
            DashboardUiComponent::Balances(balances)
        }),
        DashboardUiComponent::LatestExecutedTxs,
    ])
}
