mod namada_web_client;

use std::str::FromStr;

// we use the SDK
use namada::ledger::rpc::get_token_balance;
use namada::types::address::Address;

// we need a platform specific client
use namada_web_client::NamadaWebClient;

/// exposes an API for the Namada SDK to be used in web browsers
pub struct NamadaWebSdk {
    rpc_address: String,
}

impl NamadaWebSdk {
    /// creates a new instance with a given node rpc address
    pub fn new(rpc_address: String) -> Self {
        NamadaWebSdk {
            rpc_address: rpc_address,
        }
    }

    /// returns balance by account and token
    pub async fn get_token_balance_by_account_and_token(
        &self,
        account_address: String,
        token_address: String,
    ) -> Result<String, std::io::Error> {
        // We need to pass the SDK calls a client. This is specific to WASM web browser
        let rpc_address = self.rpc_address.clone();
        let namada_web_client = NamadaWebClient { rpc_address };

        // we need the input data in correct types
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
}
