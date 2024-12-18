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
pub struct IRegistryGetAllRequest {
    #[serde(rename = "scope")]
    pub scope: Vec<String>,
    #[serde(rename = "domain", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub domain: Option<Option<String>>,
}

impl IRegistryGetAllRequest {
    pub fn new(scope: Vec<String>) -> IRegistryGetAllRequest {
        IRegistryGetAllRequest {
            scope,
            domain: None,
        }
    }
}
