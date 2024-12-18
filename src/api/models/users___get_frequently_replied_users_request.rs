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
pub struct UsersGetFrequentlyRepliedUsersRequest {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl UsersGetFrequentlyRepliedUsersRequest {
    pub fn new(user_id: String) -> UsersGetFrequentlyRepliedUsersRequest {
        UsersGetFrequentlyRepliedUsersRequest {
            user_id,
            limit: None,
        }
    }
}

