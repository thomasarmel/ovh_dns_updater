//! This mod contains structs that allow genericity between IPv4 and IPv6

use crate::public_ip_retriever::check_ip_format::{parse_ipv4_str, parse_ipv6_str};
use crate::public_ip_retriever::public_ip_retriever_error::PublicIpRetrieverError;
use crate::public_ip_retriever::public_ip_retriever_error::PublicIpRetrieverError::*;
use std::net::{Ipv4Addr, Ipv6Addr};

pub(crate) trait GenericIpTypes {
    /// std::net::Ipv4Addr or std::net::Ipv6Addr
    type GenericIpAddr;
    /// Ip4ConnectionError or Ip6ConnectionError
    const IP_CONNECTION_ERROR: PublicIpRetrieverError;
    /// Ip4ParseError or Ip6ParseError
    const IP_PARSE_ERROR: PublicIpRetrieverError;
    /// parse_ipv4_str() or parse_ipv6_str()
    const IP_STR_PARSER: fn(&str) -> Result<Self::GenericIpAddr, PublicIpRetrieverError>;
}

pub(crate) struct Ip4Types;
pub(crate) struct Ip6Types;

impl GenericIpTypes for Ip4Types {
    type GenericIpAddr = Ipv4Addr;
    const IP_CONNECTION_ERROR: PublicIpRetrieverError = Ip4ConnectionError;
    const IP_PARSE_ERROR: PublicIpRetrieverError = Ip4ParseError;
    const IP_STR_PARSER: fn(&str) -> Result<Self::GenericIpAddr, PublicIpRetrieverError> =
        parse_ipv4_str;
}

impl GenericIpTypes for Ip6Types {
    type GenericIpAddr = Ipv6Addr;
    const IP_CONNECTION_ERROR: PublicIpRetrieverError = Ip6ConnectionError;
    const IP_PARSE_ERROR: PublicIpRetrieverError = Ip6ParseError;
    const IP_STR_PARSER: fn(&str) -> Result<Self::GenericIpAddr, PublicIpRetrieverError> =
        parse_ipv6_str;
}
