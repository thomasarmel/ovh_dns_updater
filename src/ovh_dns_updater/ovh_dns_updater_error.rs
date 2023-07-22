use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum OvhDnsUpdaterError {
    #[error("Failed to login into OVH account: maybe application doesn't exist or credentials are wrong. Otherwise check network connection")]
    OvhLoginError,
    #[error("Incorrect domain name format")]
    IncorrectDomainNameFormat,
    #[error("Domain retrieving error, check domain name zone and network connection")]
    OvhDomainRetrievingError,
    #[error("Incorrect response format from OVH API")]
    IncorrectAPIResponseFormat,
    #[error("Domain zone entry doesn't exist, check domain or subdomain name")]
    DomainZoneEntryDoesntExist,
    #[error("Domain update error, check domain name zone and network connection")]
    OvhDomainUpdatingError,
}
