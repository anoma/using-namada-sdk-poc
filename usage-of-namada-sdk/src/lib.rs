mod namada_sdk;
mod utils;
mod web_namada;

use std::str;
use wasm_bindgen::prelude::*;
use web_namada::perform_request;

// This is just for being able to call perform_request from the testing UI
// in the real situation the Namada SDK is calling
#[wasm_bindgen]
pub async fn perform_request_from_ui() {
    let path_for_retrieving_balance_for_account = "/shell/value/#atest1v4ehgw36x3prswzxggunzv6pxqmnvdj9xvcyzvpsggeyvs3cg9qnywf589qnwvfsg5erg3fkl09rg5/balance/#atest1d9khqw36xyuyxdf5g4qnjwfkxvc52vf48qu5yseexcm52wpcg56ygd3sxdp52s2z89znsvz93amfph".to_string();
    let prove = false;
    let data = Some("".to_string());
    let height = Some("0".to_string());

    let encoded_response_query =
        perform_request(path_for_retrieving_balance_for_account, prove, data, height)
            .await
            .unwrap();

    let data = encoded_response_query.data;
    let data_as_str = str::from_utf8(&data).unwrap();
    log("result at perform_request_from_ui");
    log(data_as_str);
}

#[wasm_bindgen]
extern "C" {
    // window.alert
    fn alert(s: &str);

    // console.log
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
