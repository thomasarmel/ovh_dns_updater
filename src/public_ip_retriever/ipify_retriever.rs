//! This module contains IP retriever for https://ipify.org

use crate::public_ip_retriever::generic_ip_types::{GenericIpTypes, Ip4Types, Ip6Types};
use crate::public_ip_retriever::public_ip_retriever_error::PublicIpRetrieverError;
use crate::public_ip_retriever::PublicIpRetriever;
use async_trait::async_trait;
use std::net::{Ipv4Addr, Ipv6Addr};

pub(crate) struct IpifyIpRetriever {
    client: reqwest::Client,
}

impl IpifyIpRetriever {
    const IPIFY_IP4_URL: &'static str = "https://api.ipify.org?format=json";
    const IPIFY_IP6_URL: &'static str = "https://api6.ipify.org?format=json";

    async fn get_ip_4_or_6<GenericIPTypesStruct: GenericIpTypes>(
        &self,
        ip_me_service_url: &str,
    ) -> Result<GenericIPTypesStruct::GenericIpAddr, PublicIpRetrieverError> {
        let ip_json_str = self.client.get(ip_me_service_url).send().await;
        match ip_json_str {
            Ok(ip_response) => {
                let ip_obj_result = ip_response.json::<IpifyResponse>().await;
                match ip_obj_result {
                    Ok(ip_obj) => GenericIPTypesStruct::IP_STR_PARSER(&ip_obj.ip),
                    Err(_) => Err(GenericIPTypesStruct::IP_PARSE_ERROR),
                }
            }
            Err(_) => Err(GenericIPTypesStruct::IP_CONNECTION_ERROR),
        }
    }
}

#[async_trait]
impl PublicIpRetriever for IpifyIpRetriever {
    fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn get_ip4(&self) -> Result<Ipv4Addr, PublicIpRetrieverError> {
        self.get_ip_4_or_6::<Ip4Types>(Self::IPIFY_IP4_URL).await
    }

    async fn get_ip6(&self) -> Result<Ipv6Addr, PublicIpRetrieverError> {
        self.get_ip_4_or_6::<Ip6Types>(Self::IPIFY_IP6_URL).await
    }
}

#[derive(serde::Deserialize)]
struct IpifyResponse {
    ip: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public_ip_retriever::public_ip_retriever_error::PublicIpRetrieverError::Ip6ConnectionError;

    #[tokio::test]
    async fn test_ipify_ip_retriever() {
        let ipify_ip_retriever = IpifyIpRetriever::new();
        let ip4_future = ipify_ip_retriever.get_ip4();
        let ip6_future = ipify_ip_retriever.get_ip6();

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
