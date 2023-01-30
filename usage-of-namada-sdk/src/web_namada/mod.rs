use base64::{engine::general_purpose, Engine as _};
use std::io::ErrorKind;
use std::str;
use std::str::FromStr;

use crate::namada_sdk::fake_types::EncodedResponseQuery;
use namada::ledger::queries::Client as NamadaClient;
use namada::ledger::rpc::get_token_balance;
use namada::types::address::Address;
use namada::types::storage::BlockHeight;
use serde::{Deserialize, Serialize};
use tendermint_rpc::error::Error as RpcError;
use tendermint_rpc::response::Response as TendermintResponse;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

extern crate console_error_panic_hook;
use std::panic;

#[derive(Serialize, Deserialize)]
pub struct ResponseQuerySerde {
    pub data: Vec<u8>,
    pub info: String,
    pub proof: Option<String>,
}

/// ResponseQuerySerde -> JsValue
/// This is useful for the usage in TypeScript
#[wasm_bindgen]
pub fn response_query_serde_to_js_value(
    data: Vec<u8>,
    info: String,
    proof: Option<String>,
) -> JsValue {
    let response_query_serde = ResponseQuerySerde {
        data: data,
        info: info,
        proof: None::<String>,
    };
    serde_wasm_bindgen::to_value(&response_query_serde).unwrap()
}

/// JsValue -> ResponseQuerySerde
pub fn js_value_to_response_query_serde(js_value: JsValue) -> ResponseQuerySerde {
    serde_wasm_bindgen::from_value(js_value).unwrap()
}

/// NamadaWebClient this is the web specific client that implements the Client trait from the SDK
/// The most important method is `request`, mostly otherwise anything here should be utils for
/// transforming the data and error handling
pub struct NamadaWebClient;

/// performs a network call using js. Takes Namada storage path, data
/// and other params. Performs the call and returns Borsh serialized value.
/// The decryption of the data is the task of the caller of this function
pub async fn perform_request(
    path: String,
    prove: bool,
    data: Option<String>,
    height: Option<String>,
) -> Result<EncodedResponseQuery, std::io::Error> {
    // path
    let path_as_js_value = JsValue::from_str(path.as_str());

    // data
    let data_or_empty_string = data.unwrap_or("".to_string());
    let data_as_js_value = JsValue::from_str(data_or_empty_string.as_str());

    // height
    let height_or_zero = height.unwrap_or("0".to_string());
    let height_as_js_value = JsValue::from_str(height_or_zero.as_str());

    // create network utils object, this is a class defined in the accompanying
    // TypeScript code, that is mapped to Rust in the extern "C" block
    // at the end if this file
    let networking_utils = NetworkingUtils::new();

    // we now call the foreign function to fetch the data from the network
    let response_result = networking_utils
        .rpc_call(
            path_as_js_value,
            prove,
            data_as_js_value,
            height_as_js_value,
        )
        .await;

    // if errored we return
    if response_result.is_err() {
        let response_error_maybe = response_result.err();
        let response_error = response_error_maybe.unwrap();
        let response_error_as_string_maybe = JsValue::as_string(&response_error);
        let _response_error_as_string = response_error_as_string_maybe.unwrap();
        return Err(std::io::Error::from(ErrorKind::Other));
    }

    // otherwise extract the result and return it
    //
    // data is Borsh serialized and base64 encoded as it came from network
    // however we need to pass it back to the SDK in Borsh serialized byte array
    // example when fetching the balance, the network returns us
    // "ABCl1OgAAAA="
    // which translates to
    // Amount {micro: 1000000}
    // in Rust
    let response = response_result.unwrap();
    // we turn the data to EncodedResponseQuery
    let response_temporary_object = js_value_to_response_query_serde(response);
    // but the data field contains byte array but as base64 encoded, we need non base64 encoded
    let data_as_str = std::str::from_utf8(&response_temporary_object.data);
    // now we have data in string
    let borsh_encoded_byte_array = general_purpose::STANDARD
        .decode(data_as_str.unwrap())
        .unwrap();
    // now it is not base64 encoded anymore
    let encoded_response_query = EncodedResponseQuery {
        data: borsh_encoded_byte_array,
        info: response_temporary_object.info,
        proof: None,
    };
    Ok(encoded_response_query)
}

/// wraps SDK usage for balance fetching and makes it usable by strings
pub async fn get_token_balance_by_account_and_token(
    account_address: String,
    token_address: String,
) -> Result<String, std::io::Error> {
    // prepare parameters
    let namada_web_client = NamadaWebClient;
    let account_address = Address::from_str(account_address.as_str()).unwrap();
    let token_address = Address::from_str(token_address.as_str()).unwrap();

    // performing the query using a function from the SDK
    let token_balance_maybe =
        get_token_balance(&namada_web_client, &account_address, &token_address).await;

    // prepare the return data
    let token_balance = token_balance_maybe.unwrap();
    let token_balance_as_string = token_balance.to_string();
    Ok(token_balance_as_string)
}

/// this implements the required method in `Client` train
/// The main responsibility to transform input and output data
#[async_trait::async_trait(?Send)]
impl NamadaClient for NamadaWebClient {
    type Error = std::io::Error;

    // the SDK expects this to be implemented to do the actual network
    // calls in a platform specific way
    async fn request(
        &self,
        path: String,
        data: Option<Vec<u8>>,
        _height: Option<BlockHeight>,
        prove: bool,
    ) -> Result<EncodedResponseQuery, Self::Error> {
        // transform block height
        let height_as_string = 0.to_string();

        // transform data
        let data_as_string_maybe = match data {
            Some(data) => {
                let data_as_str = str::from_utf8(&data);
                match data_as_str {
                    Ok(data_as_str) => Some(data_as_str.to_string()),
                    Err(_) => None,
                }
            }
            None => None,
        };

        // we call a node and expect Borsh encode data
        let call_result =
            perform_request(path, prove, data_as_string_maybe, Some(height_as_string)).await;

        // if it errored we return here
        if call_result.is_err() {
            return Err(std::io::Error::from(ErrorKind::Other));
        }

        let encoded_response_query = call_result.unwrap();
        Ok(encoded_response_query)
    }

    async fn perform<R>(&self, request: R) -> Result<R::Response, RpcError>
    where
        R: tendermint_rpc::SimpleRequest,
    {
        let request_json = request.into_json();
        log("perform callback called with:");
        log(request_json.as_str());
        TendermintResponse::from_string("response")
    }
}

// some js utils
#[wasm_bindgen(raw_module = "./networkingUtils")]
extern "C" {
    #[wasm_bindgen(js_class = "NetworkingUtils")]
    type NetworkingUtils;

    #[wasm_bindgen(constructor)]
    fn new() -> NetworkingUtils;

    /// this is the signature of the js method that is used to perform
    /// the calls to the chain
    /// `JsValue` here are `string`s in TypeScript
    /// we have to do this conversion  manually
    #[wasm_bindgen(catch, method, js_class = "NetworkingUtils", js_name = "rpcCall")]
    async fn rpc_call(
        this: &NetworkingUtils,
        path: JsValue,
        prove: bool,
        data: JsValue,
        height: JsValue,
    ) -> Result<JsValue, JsValue>;

}

// some js utils
#[wasm_bindgen]
extern "C" {
    // console.log
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
