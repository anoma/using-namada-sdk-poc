// these are also just to mock what will actually be used from Namada SDK
use namada::ledger::queries::ResponseQuery;

/// [`ResponseQuery`] with borsh-encoded `data` field
pub type EncodedResponseQuery = ResponseQuery<Vec<u8>>;
