use std::io::ErrorKind;
use std::str;

use crate::namada_sdk::fake_types::{BlockHeight, EncodedResponseQuery, ResponseQuery};
use crate::namada_sdk::NamadaClient;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
pub struct ResponseQuerySerde {
    pub data: Vec<u8>,
    pub info: String,
    pub proof: Option<String>,
}

/// ResponseQuerySerde -> JsValue
#[wasm_bindgen]
pub fn response_query_serde_to_js_value(
    data: Vec<u8>,
    info: String,
    proof: Option<String>,
) -> JsValue {
    let response_query_serde = ResponseQuerySerde {
        data: data,
        info: info,
        proof: proof,
    };
    serde_wasm_bindgen::to_value(&response_query_serde).unwrap()
}

/// JsValue -> ResponseQuerySerde
pub fn js_value_to_response_query_serde(js_value: JsValue) -> ResponseQuerySerde {
    serde_wasm_bindgen::from_value(js_value).unwrap()
}

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

    // create network utils object
    let networking_utils = NetworkingUtils::new();

    // TODO: we actually need more than just the data, we need info
    let response_future = networking_utils.rpc_call(
        path_as_js_value,
        prove,
        data_as_js_value,
        height_as_js_value,
    );
    let response_result = response_future.await;

    if let Ok(response_as_js_value) = response_result {
        let aaa = js_value_to_response_query_serde(response_as_js_value);

        let encoded_response_query = EncodedResponseQuery {
            data: aaa.data,
            info: aaa.info,
            proof: aaa.proof,
        };
        Ok(encoded_response_query)
    } else {
        let response_error_maybe = response_result.err();
        let response_error = response_error_maybe.unwrap();
        Err(std::io::Error::from(ErrorKind::Other))
    }
}

/// this implements the required method in `Client` train
/// The main responsibility to transform input and output data
#[async_trait::async_trait(?Send)]
impl NamadaClient for NamadaWebClient {
    type Error = std::io::Error;

    /// this has to be implemented by the consumer as in web
    /// we cannot use the `HttpClient` of Tendermint
    async fn request(
        &self,
        path: String,
        data: Option<Vec<u8>>,
        height: Option<BlockHeight>,
        prove: bool,
    ) -> Result<EncodedResponseQuery, Self::Error> {
        // transform block height
        let BlockHeight(height_as_u64) = height.unwrap();
        let height_as_string = height_as_u64.to_string();

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

        // we try to extract the value from a successful call
        // TODO: we still need more than just the data
        if let Ok(call_value) = call_result {
            // let response_query = EncodedResponseQuery {
            //     data: call_value.as_bytes().to_vec(),
            //     info: "no_info".to_string(),
            //     proof: None,
            // };
            return Ok(call_value);
        }

        // else we return an error
        let call_error_maybe = call_result.err();
        let call_error = call_error_maybe.unwrap();
        let call_error_as_string = call_error.to_string();
        log("Err in request");
        log(call_error_as_string.as_str());
        Err(std::io::Error::from(ErrorKind::Other))
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
