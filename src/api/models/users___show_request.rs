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
pub struct UsersShowRequest {
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "userIds", skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// The local host is represented with `null`.
    #[serde(rename = "host", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub host: Option<Option<String>>,
}

impl UsersShowRequest {
    pub fn new() -> UsersShowRequest {
        UsersShowRequest {
            user_id: None,
            user_ids: None,
            username: None,
            host: None,
        }
    }
}

