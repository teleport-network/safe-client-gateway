use crate::routes::balances::models::Balances;
use crate::routes::safe_apps::models::SafeApp;
use crate::routes::safes::models::SafeState;
use crate::routes::transactions::models::summary::TransactionListItem;
use serde::Serialize;
use serde_json::value::RawValue;

#[derive(Serialize)]
#[serde(tag = "type", content = "content")]
pub enum DashboardUiComponent {
    HistoryTxs(Vec<TransactionListItem>),
    PendingTxs(Vec<TransactionListItem>),
    Balances(Balances),
    SafeApps(Vec<SafeApp>),
    Safe(SafeState),
    Collectibles(Box<RawValue>),
    ErrorLoadingComponent,
}
