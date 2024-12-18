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
pub struct AdminDriveFilesRequest {
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "sinceId", skip_serializing_if = "Option::is_none")]
    pub since_id: Option<String>,
    #[serde(rename = "untilId", skip_serializing_if = "Option::is_none")]
    pub until_id: Option<String>,
    #[serde(rename = "userId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Option<String>>,
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<String>>,
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<Origin>,
    /// The local host is represented with `null`.
    #[serde(rename = "hostname", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<Option<String>>,
}

impl AdminDriveFilesRequest {
    pub fn new() -> AdminDriveFilesRequest {
        AdminDriveFilesRequest {
            limit: None,
            since_id: None,
            until_id: None,
            user_id: None,
            r#type: None,
            origin: None,
            hostname: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Origin {
    #[serde(rename = "combined")]
    Combined,
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "remote")]
    Remote,
}

impl Default for Origin {
    fn default() -> Origin {
        Self::Combined
    }
}

