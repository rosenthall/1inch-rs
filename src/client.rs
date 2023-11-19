use core::fmt;
use strum_macros::{FromRepr};

/// Struct to work with 1inch api
pub struct OneInchClient {

    /// reqwest::Client does not need to be Rc/Arc because it already uses an Arc internally.
    http_client : reqwest::Client,

    /// An authorization token for interacting with API.
    /// There you can get your own token : <https://portal.1inch.dev/applications>
    pub(crate) token : String,

    /// The ID of the network on which you want to work.
    /// You can interact only with 1 specified network with your client.
    pub(crate) network_id : SupportedNetworks,
}



/// Function creates a OneInchClient instance with default http settings.
pub fn new_with_default_http(token : String, network_id : SupportedNetworks) -> OneInchClient {
    OneInchClient {
        http_client: reqwest::Client::default(),
        token,
        network_id,
    }
}



/// List of all supported Networks/Chains with their IDs.
#[derive(FromRepr, Debug, Copy, Clone)]
#[repr(u32)]
pub enum SupportedNetworks {
    Ethereum = 1,
    Optimism = 10,
    BSC = 56,
    Gnosis = 100,
    Polygon = 137,
    Fantom = 250,
    ZkSync = 324,
    Klaytn = 8217,
    Base = 8453,
    Arbitrum = 42161,
    Avalanche = 43114,
    Aurora = 1313161554,
}

impl fmt::Display for SupportedNetworks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u32)
    }
}
