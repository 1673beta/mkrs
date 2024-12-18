/*
 * Misskey API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2024.9.0-alpha.4
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetaDetailedOnly {
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub features: Option<Box<models::MetaDetailedOnlyFeatures>>,
    #[serde(rename = "proxyAccountName", deserialize_with = "Option::deserialize")]
    pub proxy_account_name: Option<String>,
    #[serde(rename = "requireSetup")]
    pub require_setup: bool,
    #[serde(rename = "cacheRemoteFiles")]
    pub cache_remote_files: bool,
    #[serde(rename = "cacheRemoteSensitiveFiles")]
    pub cache_remote_sensitive_files: bool,
}

impl MetaDetailedOnly {
    pub fn new(proxy_account_name: Option<String>, require_setup: bool, cache_remote_files: bool, cache_remote_sensitive_files: bool) -> MetaDetailedOnly {
        MetaDetailedOnly {
            features: None,
            proxy_account_name,
            require_setup,
            cache_remote_files,
            cache_remote_sensitive_files,
        }
    }
}

