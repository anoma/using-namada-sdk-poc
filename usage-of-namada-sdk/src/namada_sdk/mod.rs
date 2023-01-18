pub mod fake_types;

use fake_types::EncodedResponseQuery;
use namada::types::storage::BlockHeight;

// This is containing a trait similar to the one that is to be exported from
// Namada SDK. I just made this mock version to be able to run the js side

#[async_trait::async_trait(?Send)]
pub trait NamadaClient {
    /// `std::io::Error` can happen in decoding with
    /// `BorshDeserialize::try_from_slice`
    type Error: From<std::io::Error>;

    /// Send a simple query request at the given path. For more options, use the
    /// `request` method.
    async fn simple_request(&self, path: String) -> Result<Vec<u8>, Self::Error> {
        self.request(path, None, None, false)
            .await
            .map(|response| response.data)
    }

    async fn request(
        &self,
        path: String,
        data: Option<Vec<u8>>,
        height: Option<BlockHeight>,
        prove: bool,
    ) -> Result<EncodedResponseQuery, Self::Error>;
}

// pub async fn query_epoch<C: Client + NamadaClient + Sync>(client: &C) -> Epoch {
// this is the original, it needs to satisfy Client, which is
// use tendermint_rpc::Client;
// pub async fn query_epoch<C: NamadaClient + Sync>(client: &C) -> Epoch {
//     Epoch(1)
// }
