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
pub struct InviteCode {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "expiresAt", deserialize_with = "Option::deserialize")]
    pub expires_at: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "createdBy")]
    pub created_by: Box<models::UserLite>,
    #[serde(rename = "usedBy")]
    pub used_by: Box<models::UserLite>,
    #[serde(rename = "usedAt", deserialize_with = "Option::deserialize")]
    pub used_at: Option<String>,
    #[serde(rename = "used")]
    pub used: bool,
}

impl InviteCode {
    pub fn new(id: String, code: String, expires_at: Option<String>, created_at: String, created_by: models::UserLite, used_by: models::UserLite, used_at: Option<String>, used: bool) -> InviteCode {
        InviteCode {
            id,
            code,
            expires_at,
            created_at,
            created_by: Box::new(created_by),
            used_by: Box::new(used_by),
            used_at,
            used,
        }
    }
}
