use std::time::SystemTime;
use jni::objects::{JObject, JValue};
use jni::JNIEnv;
use rust_decimal::prelude::ToPrimitive;
use crate::asb::asb_btc_balance_data::AsbBtcBalanceData;
use crate::asb::asb_xmr_balance_data::AsbXmrBalanceData;

pub(crate) fn on_xmr_lock_confirmation(env: &JNIEnv, txid: String, confirmations: u64) {
    let listener = get_swap_listener(&env);
    let txid_bytes = JObject::from(env.byte_array_from_slice(txid.as_bytes()).expect("Failed to get swap_id bytes"));
    let confs = JValue::from(confirmations.to_i64().expect("Failed to get confirmations int64"));
    if let JValue::Object(listener) = listener {
        let result = env.call_method(listener, "onXmrLockConfirmation", "([BJ)V", &[JValue::from(txid_bytes), confs]);
    }
}

pub(crate) fn on_asb_xmr_balance_data(env: &JNIEnv, data: AsbXmrBalanceData) {
    let listener = get_asb_listener(&env);
    let asb_balance_data_json = serde_json::to_string(&data).unwrap();
    let balance_json_bytes = JObject::from(env.byte_array_from_slice(asb_balance_data_json.as_bytes()).expect("Failed to get swap_id bytes"));
    if let JValue::Object(listener) = listener {
        let result = env.call_method(listener, "onAsbXmrBalanceData", "([B)V", &[JValue::from(balance_json_bytes)]);
    }
}

pub(crate) fn on_asb_btc_balance_data(env: &JNIEnv, data: AsbBtcBalanceData) {
    let listener = get_asb_listener(&env);
    let asb_balance_data_json = serde_json::to_string(&data).unwrap();
    let balance_json_bytes = JObject::from(env.byte_array_from_slice(asb_balance_data_json.as_bytes()).expect("Failed to get swap_id bytes"));
    if let JValue::Object(listener) = listener {
        let result = env.call_method(listener, "onAsbBtcBalanceData", "([B)V", &[JValue::from(balance_json_bytes)]);
    }
}

pub(crate) fn on_xmr_rpc_download_progress(env: &JNIEnv, pct: u64) {
    let listener = get_rpc_download_listener(&env);
    let percent = JValue::from(pct.to_i64().expect("Failed to get percent int64"));
    if let JValue::Object(listener) = listener {
        let result = env.call_method(listener, "onXmrRpcDownloadProgress", "(J)V", &[percent]);
    }
}

pub(crate) fn get_swap_listener<'a>(env: &'a JNIEnv<'a>) -> JValue<'a> {
    let controller = env
        .find_class("swap/gui/controller/SwapsController")
        .expect("Failed to load the target class");
    env.get_static_field(controller, "swapListener", "Lswap/listener/SwapListener;").unwrap()
}

pub(crate) fn get_asb_listener<'a>(env: &'a JNIEnv<'a>) -> JValue<'a> {
    let controller = env
        .find_class("swap/gui/controller/SwapsController")
        .expect("Failed to load the target class");
    env.get_static_field(controller, "asbListener", "Lswap/listener/AsbListener;").unwrap()
}

pub(crate) fn get_rpc_download_listener<'a>(env: &'a JNIEnv<'a>) -> JValue<'a> {
    let controller = env
        .find_class("swap/gui/controller/PairingController")
        .expect("Failed to load the target class");
    env.get_static_field(controller, "rpcDownloadListener", "Lswap/listener/RpcDownloadListener;").unwrap()
}

pub fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}