use std::fmt::Display;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub(crate) trait GenericIpAddr {
    /// std::net::Ipv4Addr or std::net::Ipv6Addr
    type IpAddrStruct: FromStr + Display;
    /// "A" or "AAAA"
    const DNS_ENTRY_FORMAT: &'static str;
}

pub(crate) struct Ip4AddrStruct;
pub(crate) struct Ip6AddrStruct;

impl GenericIpAddr for Ip4AddrStruct {
    type IpAddrStruct = Ipv4Addr;
    const DNS_ENTRY_FORMAT: &'static str = "A";
}

impl GenericIpAddr for Ip6AddrStruct {
    type IpAddrStruct = Ipv6Addr;
    const DNS_ENTRY_FORMAT: &'static str = "AAAA";
}
