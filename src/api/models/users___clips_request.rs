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
pub struct UsersClipsRequest {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "sinceId", skip_serializing_if = "Option::is_none")]
    pub since_id: Option<String>,
    #[serde(rename = "untilId", skip_serializing_if = "Option::is_none")]
    pub until_id: Option<String>,
}

impl UsersClipsRequest {
    pub fn new(user_id: String) -> UsersClipsRequest {
        UsersClipsRequest {
            user_id,
            limit: None,
            since_id: None,
            until_id: None,
        }
    }
}

