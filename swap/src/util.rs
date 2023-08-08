use jni::objects::{JObject, JValue};
use jni::JNIEnv;
use rust_decimal::prelude::ToPrimitive;

pub(crate) fn on_xmr_lock_confirmation(env: &JNIEnv, txid: String, confirmations: u64) {
    let listener = get_swap_listener(&env);
    let txid_bytes = JObject::from(env.byte_array_from_slice(txid.as_bytes()).expect("Failed to get swap_id bytes"));
    let confs = JValue::from(confirmations.to_i64().expect("Failed to get confirmations int64"));
    if let JValue::Object(listener) = listener {
        let result = env.call_method(listener, "onXmrLockConfirmation", "([BJ)V", &[JValue::from(txid_bytes), confs]);
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
        .find_class("swap/gui/SwapsController")
        .expect("Failed to load the target class");
    env.get_static_field(controller, "swapListener", "Lswap/listener/SwapListener;").unwrap()
}

pub(crate) fn get_rpc_download_listener<'a>(env: &'a JNIEnv<'a>) -> JValue<'a> {
    let controller = env
        .find_class("swap/gui/PairingController")
        .expect("Failed to load the target class");
    env.get_static_field(controller, "rpcDownloadListener", "Lswap/listener/RpcDownloadListener;").unwrap()
}