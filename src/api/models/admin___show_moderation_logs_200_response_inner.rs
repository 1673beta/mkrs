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
pub struct AdminShowModerationLogs200ResponseInner {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "info")]
    pub info: serde_json::Value,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "user")]
    pub user: models::UserDetailedNotMe,
}

impl AdminShowModerationLogs200ResponseInner {
    pub fn new(id: String, created_at: String, r#type: String, info: serde_json::Value, user_id: String, user: models::UserDetailedNotMe) -> AdminShowModerationLogs200ResponseInner {
        AdminShowModerationLogs200ResponseInner {
            id,
            created_at,
            r#type,
            info,
            user_id,
            user,
        }
    }
}
