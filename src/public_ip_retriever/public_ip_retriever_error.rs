use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum PublicIpRetrieverError {
    #[error("Failed to establish IPv4 connection")]
    Ip4ConnectionError,
    #[error("Failed to establish IPv6 connection")]
    Ip6ConnectionError,
    #[error("Failed to parse IPv4 address")]
    Ip4ParseError,
    #[error("Failed to parse IPv6 address")]
    Ip6ParseError,
}
