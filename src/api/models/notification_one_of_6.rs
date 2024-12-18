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
pub struct NotificationOneOf6 {
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
    #[serde(rename = "note")]
    pub note: models::Note,
}

impl NotificationOneOf6 {
    pub fn new(id: String, created_at: String, r#type: Type, user: models::UserLite, user_id: String, note: models::Note) -> NotificationOneOf6 {
        NotificationOneOf6 {
            id,
            created_at,
            r#type,
            user,
            user_id,
            note,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "pollEnded")]
    PollEnded,
}

impl Default for Type {
    fn default() -> Type {
        Self::PollEnded
    }
}

