//! This module contains IP retriever for https://ident.me

use crate::public_ip_retriever::generic_ip_types::{GenericIpTypes, Ip4Types, Ip6Types};
use crate::public_ip_retriever::public_ip_retriever_error::PublicIpRetrieverError;
use crate::public_ip_retriever::PublicIpRetriever;
use async_trait::async_trait;
use std::net::{Ipv4Addr, Ipv6Addr};

pub(crate) struct IdentMeIpRetriever {
    client: reqwest::Client,
}

impl IdentMeIpRetriever {
    const INDENT_ME_IP4_URL: &'static str = "https://v4.ident.me/";
    const INDENT_ME_IP6_URL: &'static str = "https://v6.ident.me/";

    async fn get_ip_4_or_6<GenericIPTypesStruct: GenericIpTypes>(
        &self,
        ip_me_service_url: &str,
    ) -> Result<GenericIPTypesStruct::GenericIpAddr, PublicIpRetrieverError> {
        let ip_response_result = self.client.get(ip_me_service_url).send().await;
        match ip_response_result {
            Ok(ip_response) => {
                let ip_result = ip_response.text().await;
                match ip_result {
                    Ok(ip_str) => GenericIPTypesStruct::IP_STR_PARSER(&ip_str),
                    Err(_) => Err(GenericIPTypesStruct::IP_PARSE_ERROR),
                }
            }
            Err(_) => Err(GenericIPTypesStruct::IP_CONNECTION_ERROR),
        }
    }
}

#[async_trait]
impl PublicIpRetriever for IdentMeIpRetriever {
    fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn get_ip4(&self) -> Result<Ipv4Addr, PublicIpRetrieverError> {
        self.get_ip_4_or_6::<Ip4Types>(Self::INDENT_ME_IP4_URL)
            .await
    }

    async fn get_ip6(&self) -> Result<Ipv6Addr, PublicIpRetrieverError> {
        self.get_ip_4_or_6::<Ip6Types>(Self::INDENT_ME_IP6_URL)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public_ip_retriever::public_ip_retriever_error::PublicIpRetrieverError::*;

    #[tokio::test]
    async fn test_ident_me_ip_retriever() {
        let ident_me_ip_retriever = IdentMeIpRetriever::new();
        let ip4_future = ident_me_ip_retriever.get_ip4();
        let ip6_future = ident_me_ip_retriever.get_ip6();

        let ip4_result = ip4_future.await;
        let ip6_result = ip6_future.await;
        assert!(ip4_result.is_ok());
        match ip6_result {
            Ok(_) => (),
            Err(Ip6ConnectionError) => (), // If test environment does not support IPv6, this is OK
            Err(other) => panic!("Ip6 parsing error: {:?}", other),
        }
    }
}
