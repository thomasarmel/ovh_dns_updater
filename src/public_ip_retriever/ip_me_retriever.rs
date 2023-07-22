//! This module contains IP retriever for https://ip4.me and https://ip6only.me

use crate::public_ip_retriever::generic_ip_types::{GenericIpTypes, Ip4Types, Ip6Types};
use crate::public_ip_retriever::public_ip_retriever_error::PublicIpRetrieverError;
use crate::public_ip_retriever::PublicIpRetriever;
use async_trait::async_trait;
use std::net::{Ipv4Addr, Ipv6Addr};
use thiserror::Error;

pub(crate) struct IpMeIpRetriever {
    client: reqwest::Client,
}

impl IpMeIpRetriever {
    const IP_ME_IP4_URL: &'static str = "https://ip4.me/api/";
    const IP_ME_IP6_URL: &'static str = "https://ip6only.me/api/";
    // IPv[...],[IP],v1.1,,,[freetext]
    const IP_FIELD_COMA_SEPARATION_INDEX: usize = 1;

    /// Retrieve the IP from the coma separated text returned by the API
    fn parse_coma_separated_fields(text: &str) -> Result<&str, IpMeRetrieverError> {
        let fields: Vec<&str> = text.split(',').collect();
        if fields.len() > Self::IP_FIELD_COMA_SEPARATION_INDEX {
            Ok(fields[Self::IP_FIELD_COMA_SEPARATION_INDEX])
        } else {
            Err(IpMeRetrieverError::ComaSeparatedFieldsError)
        }
    }

    async fn get_ip_4_or_6<GenericIPTypesStruct: GenericIpTypes>(
        &self,
        ip_me_service_url: &str,
    ) -> Result<GenericIPTypesStruct::GenericIpAddr, PublicIpRetrieverError> {
        let ip_response_result = self.client.get(ip_me_service_url).send().await;
        let fields_coma_str = ip_response_result
            .map_err(|_| GenericIPTypesStruct::IP_CONNECTION_ERROR)?
            .text()
            .await
            .map_err(|_| GenericIPTypesStruct::IP_PARSE_ERROR)?;
        let ip_str = Self::parse_coma_separated_fields(&fields_coma_str)
            .map_err(|_| GenericIPTypesStruct::IP_PARSE_ERROR)?;
        GenericIPTypesStruct::IP_STR_PARSER(ip_str)
    }
}

#[async_trait]
impl PublicIpRetriever for IpMeIpRetriever {
    fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn get_ip4(&self) -> Result<Ipv4Addr, PublicIpRetrieverError> {
        self.get_ip_4_or_6::<Ip4Types>(IpMeIpRetriever::IP_ME_IP4_URL)
            .await
    }

    async fn get_ip6(&self) -> Result<Ipv6Addr, PublicIpRetrieverError> {
        self.get_ip_4_or_6::<Ip6Types>(IpMeIpRetriever::IP_ME_IP6_URL)
            .await
    }
}

#[derive(Error, Debug)]
pub enum IpMeRetrieverError {
    #[error("Failed to split coma separated fields")]
    ComaSeparatedFieldsError,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public_ip_retriever::public_ip_retriever_error::PublicIpRetrieverError::*;

    #[tokio::test]
    async fn test_ip_me_ip_retriever() {
        let ip_me_ip_retriever = IpMeIpRetriever::new();
        let ip4_future = ip_me_ip_retriever.get_ip4();
        let ip6_future = ip_me_ip_retriever.get_ip6();

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
