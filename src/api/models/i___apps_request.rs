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
pub struct IAppsRequest {
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort>,
}

impl IAppsRequest {
    pub fn new() -> IAppsRequest {
        IAppsRequest {
            sort: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Sort {
    #[serde(rename = "+createdAt")]
    PlusCreatedAt,
    #[serde(rename = "-createdAt")]
    CreatedAt,
    #[serde(rename = "+lastUsedAt")]
    PlusLastUsedAt,
    #[serde(rename = "-lastUsedAt")]
    LastUsedAt,
}

impl Default for Sort {
    fn default() -> Sort {
        Self::PlusCreatedAt
    }
}

