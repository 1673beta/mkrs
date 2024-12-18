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
pub struct NotificationOneOf8 {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "user")]
    pub user: models::UserLite,
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl NotificationOneOf8 {
    pub fn new(id: String, created_at: String, r#type: Type, user: models::UserLite, user_id: String) -> NotificationOneOf8 {
        NotificationOneOf8 {
            id,
            created_at,
            r#type,
            user,
            user_id,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "receiveFollowRequest")]
    ReceiveFollowRequest,
}

impl Default for Type {
    fn default() -> Type {
        Self::ReceiveFollowRequest
    }
}
