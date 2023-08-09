use serde::{Serialize};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AsbXmrBalanceData {
    pub total: u64,
    pub unlocked: u64,
    pub error: String
}