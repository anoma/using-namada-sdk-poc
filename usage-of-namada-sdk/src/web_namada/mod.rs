use std::io::ErrorKind;
use std::str;
use std::str::FromStr;

use crate::namada_sdk::fake_types::EncodedResponseQuery;
use crate::namada_sdk::NamadaClient;
use namada::ledger::rpc::get_token_balance;
use namada::types::address::Address;
use namada::types::storage::BlockHeight;
use serde::{Deserialize, Serialize};
use tendermint_rpc::{Client as TendermintClient, Error, SimpleRequest};
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
pub fn _response_query_serde_to_js_value(
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

    // we now call the foreign function to fetch the data from the network
    let response_future = networking_utils.rpc_call(
        path_as_js_value,
        prove,
        data_as_js_value,
        height_as_js_value,
    );
    let response_result = response_future.await;

    // we try to extract the data from it
    if let Ok(response_as_js_value) = response_result {
        let response_temporary_object = js_value_to_response_query_serde(response_as_js_value);

        let encoded_response_query = EncodedResponseQuery {
            data: response_temporary_object.data,
            info: response_temporary_object.info,
            proof: None,
        };
        Ok(encoded_response_query)
    } else {
        let response_error_maybe = response_result.err();
        let response_error = response_error_maybe.unwrap();
        let response_error_as_string_maybe = JsValue::as_string(&response_error);
        let response_error_as_string = response_error_as_string_maybe.unwrap();
        log("Err in perform_request");
        log(response_error_as_string.as_str());
        Err(std::io::Error::from(ErrorKind::Other))
    }
}

pub async fn perform_request_helper(rpc_payload: String) -> Result<String, std::io::Error> {
    // create network utils object
    let networking_utils = NetworkingUtils::new();

    // we now call the foreign function to fetch the data from the network
    let rpc_payload_as_js_value = JsValue::from_str(rpc_payload.as_str());
    let response_future = networking_utils.rpc_call_with_stringified_json(rpc_payload_as_js_value);
    let response_result = response_future.await;

    // we try to extract the data from it
    if let Ok(response_as_js_value) = response_result {
        let response_value_maybe = response_as_js_value.as_string();
        let response_value = response_value_maybe.unwrap();
        Ok(response_value)
    } else {
        let response_error_maybe = response_result.err();
        let response_error = response_error_maybe.unwrap();
        let response_error_as_string_maybe = JsValue::as_string(&response_error);
        let response_error_as_string = response_error_as_string_maybe.unwrap();
        log("Err in perform_request");
        log(response_error_as_string.as_str());
        Err(std::io::Error::from(ErrorKind::Other))
    }
}

/// this is wrapping the usage of get_token_balance from the SDK
pub async fn get_token_balance_wrapper(
    account_address: String,
    token_address: String,
) -> Result<(), std::io::Error> {
    let namada_web_client = NamadaWebClient;
    let account_address = Address::from_str(account_address.as_str()).unwrap();
    let token_address = Address::from_str(token_address.as_str()).unwrap();
    let token_balance_maybe =
        get_token_balance(&namada_web_client, &account_address, &token_address).await;
    let token_balance = token_balance_maybe.unwrap();
    let token_balance = token_balance.to_string();
    log(format!("token_balance: {token_balance}").as_str());
    Ok(())
}

#[async_trait::async_trait]
impl TendermintClient for NamadaWebClient {
    async fn perform<R>(&self, request: R) -> Result<R::Response, Error>
    where
        R: SimpleRequest,
    {
        log("now we should call the js callback");
        // so here we would like to perform that async call to js
        // to make the network request, if you uncomment the next lines you will see the problem
        // let json = request.into_json();
        // let networking_utils = NetworkingUtils::new();
        // let json_as_js_value = JsValue::from_str(json.as_str());
        // let response = perform_request_helper(json).await;
        tendermint_rpc::response::Response::from_string("response.unwrap()")
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

        if let Ok(call_value) = call_result {
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

    #[wasm_bindgen(
        catch,
        method,
        js_class = "NetworkingUtils",
        js_name = "rpcCallWithStringifiedJson"
    )]
    async fn rpc_call_with_stringified_json(
        this: &NetworkingUtils,
        abci_query_payload_json: JsValue,
    ) -> Result<JsValue, JsValue>;
}

// some js utils
#[wasm_bindgen]
extern "C" {
    // console.log
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
