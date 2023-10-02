use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapError {
    pub swap_id: String,
    pub error_type: ErrorType,
    pub error_message: String,
    pub fatal: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub enum  ErrorType {
    SellerInsufficientBalance,
    SwapPunished,
    FailedToInitBitcoinWallet,
    FailedToInitRefundBitcoinWallet,
    FailedToInitMoneroWallet,
    EventLoopPanic,
    DeserializeCborError,
    FailedToInitAsbMoneroWallet,
    FailedToInitAsbBitcoinWallet,
    FailedToLoadSwapDatabase,
    FailedToLoadAsbDatabase,
    SwapDisconnected,
    UnknownError
}