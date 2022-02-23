use serde::Serialize;

#[derive(Serialize)]
pub enum DashboardUiComponent {
    LatestExecutedTxs,
    PendingTxs,
    TokenBalances,
    SafeApps,
    Safe,
    NFTSs,
}
