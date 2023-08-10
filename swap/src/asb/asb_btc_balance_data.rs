use serde::{Serialize};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AsbBtcBalanceData {
    pub balance: u64,
    pub error: String
}