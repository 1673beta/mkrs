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
pub struct NotesCreateRequestPoll {
    #[serde(rename = "choices")]
    pub choices: Vec<String>,
    #[serde(rename = "multiple", skip_serializing_if = "Option::is_none")]
    pub multiple: Option<bool>,
    #[serde(rename = "expiresAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Option<i32>>,
    #[serde(rename = "expiredAfter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expired_after: Option<Option<i32>>,
}

impl NotesCreateRequestPoll {
    pub fn new(choices: Vec<String>) -> NotesCreateRequestPoll {
        NotesCreateRequestPoll {
            choices,
            multiple: None,
            expires_at: None,
            expired_after: None,
        }
    }
}

