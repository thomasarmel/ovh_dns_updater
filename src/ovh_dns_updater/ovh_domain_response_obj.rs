//! Represents the response from the OVH JSON API when requesting a domain

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case, dead_code)]
pub(crate) struct OvhDomainResponseObj {
    pub(crate) subDomain: String,
    pub(crate) zone: String,
    pub(crate) fieldType: String,
    pub(crate) id: usize,
    pub(crate) ttl: usize,
    pub(crate) target: String,
}
