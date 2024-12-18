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
pub struct NotificationOneOf10 {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "role")]
    pub role: models::Role,
}

impl NotificationOneOf10 {
    pub fn new(id: String, created_at: String, r#type: Type, role: models::Role) -> NotificationOneOf10 {
        NotificationOneOf10 {
            id,
            created_at,
            r#type,
            role,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "roleAssigned")]
    RoleAssigned,
}

impl Default for Type {
    fn default() -> Type {
        Self::RoleAssigned
    }
}
