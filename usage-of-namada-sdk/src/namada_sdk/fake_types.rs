// these are also just to mock what will actually be used from Namada SDK

pub struct BlockHeight(pub u64);

pub struct Epoch(pub u64);

#[derive(Clone, Debug, Default)]
pub struct ResponseQuery<T> {
    /// Response data to be borsh encoded
    pub data: T,
    /// Non-deterministic log of the request execution
    pub info: String,
    /// Optional proof - used for storage value reads which request `prove`
    pub proof: Option<String>,
}

/// [`ResponseQuery`] with borsh-encoded `data` field
pub type EncodedResponseQuery = ResponseQuery<Vec<u8>>;
