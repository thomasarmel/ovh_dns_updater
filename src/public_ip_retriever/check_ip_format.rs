use crate::public_ip_retriever::public_ip_retriever_error::PublicIpRetrieverError;
use crate::public_ip_retriever::public_ip_retriever_error::PublicIpRetrieverError::*;
use std::net::{Ipv4Addr, Ipv6Addr};

/// Parse an IPv4 address from a string.
pub(crate) fn parse_ipv4_str(ip: &str) -> Result<Ipv4Addr, PublicIpRetrieverError> {
    match ip.parse::<Ipv4Addr>() {
        Ok(ip) => Ok(ip),
        Err(_) => Err(Ip4ParseError),
    }
}

/// Parse an IPv6 address from a string.
pub(crate) fn parse_ipv6_str(ip: &str) -> Result<Ipv6Addr, PublicIpRetrieverError> {
    match ip.parse::<Ipv6Addr>() {
        Ok(ip) => Ok(ip),
        Err(_) => Err(Ip6ParseError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ipv4_str() {
        let ip = "1.1.1.1";
        let ip = parse_ipv4_str(ip);
        assert!(ip.is_ok());
        let ip = ip.unwrap();
        assert_eq!(ip, Ipv4Addr::new(1, 1, 1, 1));
        let ip = "1.1.1.1a";
        let ip = parse_ipv4_str(ip);
        assert!(ip.is_err());
    }

    #[test]
    fn test_parse_ipv6_str() {
        let ip = "2001:0db8:85a3::8a2e:0370:7334";
        let ip = parse_ipv6_str(ip);
        assert!(ip.is_ok());
        let ip = ip.unwrap();
        assert_eq!(
            ip,
            Ipv6Addr::new(0x2001, 0xdb8, 0x85a3, 0, 0, 0x8a2e, 0x370, 0x7334)
        );
        let ip = "2001:0db8:85a3:0000:0000:8a2e:0370:7334a";
        let ip = parse_ipv6_str(ip);
        assert!(ip.is_err());
    }
}
