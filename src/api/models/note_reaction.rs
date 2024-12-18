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
pub struct NoteReaction {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "user")]
    pub user: models::UserLite,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl NoteReaction {
    pub fn new(id: String, created_at: String, user: models::UserLite, r#type: String) -> NoteReaction {
        NoteReaction {
            id,
            created_at,
            user,
            r#type,
        }
    }
}

