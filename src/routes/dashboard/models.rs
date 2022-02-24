use crate::routes::balances::models::Balances;
use crate::routes::safe_apps::models::SafeApp;
use crate::routes::safes::models::SafeState;
use serde::Serialize;
use serde_json::value::RawValue;

#[derive(Serialize)]
#[serde(tag = "type", content = "content")]
pub enum DashboardUiComponent {
    LatestExecutedTxs,
    PendingTxs,
    Balances(Balances),
    SafeApps(Vec<SafeApp>),
    Safe(SafeState),
    Collectibles(Box<RawValue>),
    ErrorLoadingComponent,
}
