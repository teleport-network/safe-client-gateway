use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type", content = "content")]
pub enum DashboardUiComponent {
    LatestExecutedTxs,
    PendingTxs,
    TokenBalances,
    SafeApps,
    Safe,
    NFTSs,
}
