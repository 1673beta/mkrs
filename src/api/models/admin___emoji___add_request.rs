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
pub struct AdminEmojiAddRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// Use `null` to reset the category.
    #[serde(rename = "category", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub category: Option<Option<String>>,
    #[serde(rename = "aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "license", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub license: Option<Option<String>>,
    #[serde(rename = "isSensitive", skip_serializing_if = "Option::is_none")]
    pub is_sensitive: Option<bool>,
    #[serde(rename = "localOnly", skip_serializing_if = "Option::is_none")]
    pub local_only: Option<bool>,
    #[serde(rename = "roleIdsThatCanBeUsedThisEmojiAsReaction", skip_serializing_if = "Option::is_none")]
    pub role_ids_that_can_be_used_this_emoji_as_reaction: Option<Vec<String>>,
}

impl AdminEmojiAddRequest {
    pub fn new(name: String, file_id: String) -> AdminEmojiAddRequest {
        AdminEmojiAddRequest {
            name,
            file_id,
            category: None,
            aliases: None,
            license: None,
            is_sensitive: None,
            local_only: None,
            role_ids_that_can_be_used_this_emoji_as_reaction: None,
        }
    }
}

