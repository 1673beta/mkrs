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
pub struct AdminEmojiCopyRequest {
    #[serde(rename = "emojiId")]
    pub emoji_id: String,
}

impl AdminEmojiCopyRequest {
    pub fn new(emoji_id: String) -> AdminEmojiCopyRequest {
        AdminEmojiCopyRequest {
            emoji_id,
        }
    }
}
