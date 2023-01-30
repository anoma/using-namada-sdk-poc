mod namada_sdk;
mod utils;
mod web_namada;

use std::str;
use wasm_bindgen::prelude::*;
use web_namada::get_token_balance_by_account_and_token;

// This is just for being able to call perform_request from the testing UI
// in the real situation the Namada SDK is calling
// later we could have these calls being exposed to TS, that would also be our TypeScript SDK
// the only difference to this could be that we could also expose serde serialize and deserialize
// functions to pass in and get data. For now just strings.
// another difference would be the initialization with the chain data, such as URL
// see how this is being called at web_app_using_namada_sdk/src/App.tsx:fetchBalanceOfAddress
#[wasm_bindgen]
pub async fn perform_request_from_ui(address_to_query: String) -> String {
    // hardcoded NAM for now
    let nam_address =
        "atest1v4ehgw36x3prswzxggunzv6pxqmnvdj9xvcyzvpsggeyvs3cg9qnywf589qnwvfsg5erg3fkl09rg5";
    let token_balance =
        get_token_balance_by_account_and_token(nam_address.to_string(), address_to_query).await;
    token_balance.unwrap()
}

// just some utils for debugging
#[wasm_bindgen]
extern "C" {
    // window.alert
    fn alert(s: &str);

    // console.log
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
