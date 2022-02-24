use crate::routes::safe_apps::models::SafeApp;
use crate::routes::safes::models::SafeState;
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type", content = "content")]
pub enum DashboardUiComponent {
    LatestExecutedTxs,
    PendingTxs,
    TokenBalances,
    SafeApps(Vec<SafeApp>),
    Safe(SafeState),
    NFTSs,
}
