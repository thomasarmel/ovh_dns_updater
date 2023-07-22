use crate::public_ip_retriever::public_ip_retriever_error::PublicIpRetrieverError;
use async_trait::async_trait;
use futures::future;
use std::net::{Ipv4Addr, Ipv6Addr};

mod check_ip_format;
mod generic_ip_types;
mod public_ip_retriever_error;

// Other retrievers can be added here
mod ident_me_retriever;
mod ip_me_retriever;
mod ipify_retriever;
mod lafibre_info_retriever;

#[async_trait]
trait PublicIpRetriever {
    fn new() -> Self;
    async fn get_ip4(&self) -> Result<Ipv4Addr, PublicIpRetrieverError>;
    async fn get_ip6(&self) -> Result<Ipv6Addr, PublicIpRetrieverError>;
}

pub struct IpRetrieverFromFasterService {
    ip_me_retriever: ip_me_retriever::IpMeIpRetriever,
    ipify_retriever: ipify_retriever::IpifyIpRetriever,
    lafibre_info_retriever: lafibre_info_retriever::LaFibreInfoIpRetriever,
    ident_me_retriever: ident_me_retriever::IdentMeIpRetriever,
}

impl IpRetrieverFromFasterService {
    pub fn new() -> Self {
        Self {
            ip_me_retriever: ip_me_retriever::IpMeIpRetriever::new(),
            ipify_retriever: ipify_retriever::IpifyIpRetriever::new(),
            lafibre_info_retriever: lafibre_info_retriever::LaFibreInfoIpRetriever::new(),
            ident_me_retriever: ident_me_retriever::IdentMeIpRetriever::new(),
        }
    }

    pub async fn get_ip4(&self) -> Option<Ipv4Addr> {
        let ip_me_ip4_future = self.ip_me_retriever.get_ip4();
        let ipify_ip4_future = self.ipify_retriever.get_ip4();
        let lafibre_info_ip4_future = self.lafibre_info_retriever.get_ip4();
        let ident_me_ip4_future = self.ident_me_retriever.get_ip4();

        let ip4_future_select_result = future::select_ok(vec![
            ip_me_ip4_future,
            ipify_ip4_future,
            lafibre_info_ip4_future,
            ident_me_ip4_future,
        ])
        .await;

        match ip4_future_select_result {
            Ok(ip4_ok_tuple) => Some(ip4_ok_tuple.0),
            Err(_) => None,
        }
    }

    pub async fn get_ip6(&self) -> Option<Ipv6Addr> {
        let ip_me_ip6_future = self.ip_me_retriever.get_ip6();
        let ipify_ip6_future = self.ipify_retriever.get_ip6();
        let lafibre_info_ip6_future = self.lafibre_info_retriever.get_ip6();
        let ident_me_ip6_future = self.ident_me_retriever.get_ip6();

        let ip6_future_select_result = future::select_ok(vec![
            ip_me_ip6_future,
            ipify_ip6_future,
            lafibre_info_ip6_future,
            ident_me_ip6_future,
        ])
        .await;

        match ip6_future_select_result {
            Ok(ip6_ok_tuple) => Some(ip6_ok_tuple.0),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_ip4_from_faster_service() {
        let ip_retriever_from_faster_service = IpRetrieverFromFasterService::new();
        let ip4 = ip_retriever_from_faster_service.get_ip4().await;
        assert!(ip4.is_some());
    }
    // We do not test IPv6 as we don't know if it will be supported by the test environment
}
