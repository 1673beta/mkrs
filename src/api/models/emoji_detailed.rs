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
pub struct EmojiDetailed {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "aliases")]
    pub aliases: Vec<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category", deserialize_with = "Option::deserialize")]
    pub category: Option<String>,
    /// The local host is represented with `null`.
    #[serde(rename = "host", deserialize_with = "Option::deserialize")]
    pub host: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "license", deserialize_with = "Option::deserialize")]
    pub license: Option<String>,
    #[serde(rename = "isSensitive")]
    pub is_sensitive: bool,
    #[serde(rename = "localOnly")]
    pub local_only: bool,
    #[serde(rename = "roleIdsThatCanBeUsedThisEmojiAsReaction")]
    pub role_ids_that_can_be_used_this_emoji_as_reaction: Vec<String>,
}

impl EmojiDetailed {
    pub fn new(id: String, aliases: Vec<String>, name: String, category: Option<String>, host: Option<String>, url: String, license: Option<String>, is_sensitive: bool, local_only: bool, role_ids_that_can_be_used_this_emoji_as_reaction: Vec<String>) -> EmojiDetailed {
        EmojiDetailed {
            id,
            aliases,
            name,
            category,
            host,
            url,
            license,
            is_sensitive,
            local_only,
            role_ids_that_can_be_used_this_emoji_as_reaction,
        }
    }
}
