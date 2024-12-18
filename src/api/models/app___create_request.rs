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
pub struct AppCreateRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "permission")]
    pub permission: Vec<String>,
    #[serde(rename = "callbackUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<Option<String>>,
}

impl AppCreateRequest {
    pub fn new(name: String, description: String, permission: Vec<String>) -> AppCreateRequest {
        AppCreateRequest {
            name,
            description,
            permission,
            callback_url: None,
        }
    }
}

