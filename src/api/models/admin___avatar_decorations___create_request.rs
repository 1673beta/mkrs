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
pub struct AdminAvatarDecorationsCreateRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "roleIdsThatCanBeUsedThisDecoration", skip_serializing_if = "Option::is_none")]
    pub role_ids_that_can_be_used_this_decoration: Option<Vec<String>>,
}

impl AdminAvatarDecorationsCreateRequest {
    pub fn new(name: String, description: String, url: String) -> AdminAvatarDecorationsCreateRequest {
        AdminAvatarDecorationsCreateRequest {
            name,
            description,
            url,
            role_ids_that_can_be_used_this_decoration: None,
        }
    }
}
