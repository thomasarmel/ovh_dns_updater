use crate::ovh::client::OvhClient;
use crate::ovh_dns_updater::generic_ip_addr::{GenericIpAddr, Ip4AddrStruct, Ip6AddrStruct};
use crate::ovh_dns_updater::ovh_dns_updater_error::OvhDnsUpdaterError;
use crate::ovh_dns_updater::ovh_dns_updater_error::OvhDnsUpdaterError::*;
use crate::ovh_dns_updater::ovh_domain_response_obj::OvhDomainResponseObj;
use addr::parse_domain_name;
use serde_json::json;
use std::net::{Ipv4Addr, Ipv6Addr};

mod generic_ip_addr;
pub mod ovh_dns_updater_error;
mod ovh_domain_response_obj;

pub struct OvhDnsUpdater {
    ovh_client: OvhClient,
}

impl OvhDnsUpdater {
    const OVH_API_DOMAIN_ZONE_PREFIX: &'static str = "/domain/zone/";

    pub fn new(
        ovh_endpoint: &str,
        ovh_application_key: &str,
        ovh_application_secret: &str,
        ovh_consumer_key: &str,
    ) -> Result<Self, OvhDnsUpdaterError> {
        Ok(Self {
            ovh_client: OvhClient::new(
                ovh_endpoint,
                ovh_application_key,
                ovh_application_secret,
                ovh_consumer_key,
            )
            .ok_or(OvhLoginError)?,
        })
    }

    async fn get_dns_record_id<IpVersion: GenericIpAddr>(
        &self,
        full_domain: &str,
    ) -> Result<usize, OvhDnsUpdaterError> {
        let (root_domain, sub_domain) = Self::separate_root_and_sub_domain(full_domain)?;
        let ovh_domain_field_type = IpVersion::DNS_ENTRY_FORMAT;
        let ovh_list_domains_id_api_path = format!(
            "{}{}/record?fieldType={}&subDomain={}",
            Self::OVH_API_DOMAIN_ZONE_PREFIX,
            root_domain,
            ovh_domain_field_type,
            sub_domain
        );
        let domains_id_response = self
            .ovh_client
            .get(&*ovh_list_domains_id_api_path)
            .await
            .map_err(|_| OvhDomainRetrievingError)?;
        let domains_id_list = domains_id_response
            .json::<Vec<usize>>()
            .await
            .map_err(|_| IncorrectAPIResponseFormat)?;
        if domains_id_list.len() != 1 {
            return Err(DomainZoneEntryDoesntExist);
        }
        Ok(domains_id_list[0])
    }

    async fn get_dns_ip_4_or_6<IpVersion: GenericIpAddr>(
        &self,
        full_domain: &str,
    ) -> Result<IpVersion::IpAddrStruct, OvhDnsUpdaterError> {
        let domain_id = self.get_dns_record_id::<IpVersion>(full_domain).await?;
        let (root_domain, _) = Self::separate_root_and_sub_domain(full_domain)?;
        let ovh_get_domain_api_path = format!(
            "{}{}/record/{}",
            Self::OVH_API_DOMAIN_ZONE_PREFIX,
            root_domain,
            domain_id
        );
        let domain_response = self
            .ovh_client
            .get(&*ovh_get_domain_api_path)
            .await
            .map_err(|_| OvhDomainRetrievingError)?;
        let domain_response_obj = domain_response
            .json::<OvhDomainResponseObj>()
            .await
            .map_err(|_| IncorrectAPIResponseFormat)?;
        Ok(domain_response_obj
            .target
            .parse::<IpVersion::IpAddrStruct>()
            .map_err(|_| IncorrectAPIResponseFormat)?)
    }

    async fn update_dns_ip_4_or_6<IpVersion: GenericIpAddr>(
        &self,
        full_domain: &str,
        ip: IpVersion::IpAddrStruct,
    ) -> Result<(), OvhDnsUpdaterError> {
        let domain_id = self.get_dns_record_id::<IpVersion>(full_domain).await?;
        let (root_domain, sub_domain) = Self::separate_root_and_sub_domain(full_domain)?;
        let ovh_update_domain_api_path = format!(
            "{}{}/record/{}",
            Self::OVH_API_DOMAIN_ZONE_PREFIX,
            root_domain,
            domain_id
        );
        self.ovh_client
            .put(
                &*ovh_update_domain_api_path,
                &json!({
                    "subDomain": sub_domain,
                    "target": ip.to_string(),
                    "ttl": 0,
                }),
            )
            .await
            .map_err(|_| OvhDomainUpdatingError)?;
        Ok(())
    }

    pub async fn get_dns_ipv4(&self, full_domain: &str) -> Result<Ipv4Addr, OvhDnsUpdaterError> {
        Ok(self.get_dns_ip_4_or_6::<Ip4AddrStruct>(full_domain).await?)
    }

    pub async fn get_dns_ipv6(&self, full_domain: &str) -> Result<Ipv6Addr, OvhDnsUpdaterError> {
        Ok(self.get_dns_ip_4_or_6::<Ip6AddrStruct>(full_domain).await?)
    }

    /// Update the DNS record of the given domain with the given IPv4 address
    /// # Arguments
    /// * `full_domain` - The full domain name to update, e.g. "sub.example.com" will update the "sub" subdomain of "example.com"
    /// * `ipv4` - The IPv4 address to set
    /// # Returns
    /// * `Ok(())` - If the update was successful
    /// * `Err(OvhDnsUpdaterError)` - If the update failed
    pub async fn update_dns_ipv4(
        &self,
        full_domain: &str,
        ipv4: Ipv4Addr,
    ) -> Result<(), OvhDnsUpdaterError> {
        self.update_dns_ip_4_or_6::<Ip4AddrStruct>(full_domain, ipv4)
            .await
    }

    /// Update the DNS record of the given domain with the given IPv6 address
    /// # Arguments
    /// * `full_domain` - The full domain name to update, e.g. "sub.example.com" will update the "sub" subdomain of "example.com"
    /// * `ipv6` - The IPv6 address to set
    /// # Returns
    /// * `Ok(())` - If the update was successful
    /// * `Err(OvhDnsUpdaterError)` - If the update failed
    pub async fn update_dns_ipv6(
        &self,
        full_domain: &str,
        ipv6: Ipv6Addr,
    ) -> Result<(), OvhDnsUpdaterError> {
        self.update_dns_ip_4_or_6::<Ip6AddrStruct>(full_domain, ipv6)
            .await
    }

    /// Returns the root domain and the subdomain of the given full domain
    /// # Arguments
    /// * `full_domain` - The full domain name to parse, e.g. "sub.example.com" will return ("example.com", "sub")
    /// # Returns
    /// * `Ok((root_domain, sub_domain))` - If the parsing was successful
    /// * `Err(IncorrectDomainNameFormat)` - If the parsing failed
    fn separate_root_and_sub_domain(full_domain: &str) -> Result<(&str, &str), OvhDnsUpdaterError> {
        let parsed_domain =
            parse_domain_name(full_domain).map_err(|_| IncorrectDomainNameFormat)?;
        let root_domain = parsed_domain.root().ok_or(IncorrectDomainNameFormat)?;
        let prefix_domain = parsed_domain.prefix().unwrap_or("");
        Ok((root_domain, prefix_domain))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separate_root_and_sub_domain() {
        assert_eq!(
            OvhDnsUpdater::separate_root_and_sub_domain("sub.example.com").unwrap(),
            ("example.com", "sub")
        );
        assert_eq!(
            OvhDnsUpdater::separate_root_and_sub_domain("example.com").unwrap(),
            ("example.com", "")
        );
        assert_eq!(
            OvhDnsUpdater::separate_root_and_sub_domain("sub.sub.example.com").unwrap(),
            ("example.com", "sub.sub")
        );
        assert_eq!(
            OvhDnsUpdater::separate_root_and_sub_domain("sub"),
            Err(IncorrectDomainNameFormat)
        );
    }
}
