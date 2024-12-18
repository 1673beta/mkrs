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
pub struct Channel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "lastNotedAt", deserialize_with = "Option::deserialize")]
    pub last_noted_at: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    #[serde(rename = "userId", deserialize_with = "Option::deserialize")]
    pub user_id: Option<String>,
    #[serde(rename = "bannerUrl", deserialize_with = "Option::deserialize")]
    pub banner_url: Option<String>,
    #[serde(rename = "pinnedNoteIds")]
    pub pinned_note_ids: Vec<String>,
    #[serde(rename = "color")]
    pub color: String,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    #[serde(rename = "usersCount")]
    pub users_count: f64,
    #[serde(rename = "notesCount")]
    pub notes_count: f64,
    #[serde(rename = "isSensitive")]
    pub is_sensitive: bool,
    #[serde(rename = "allowRenoteToExternal")]
    pub allow_renote_to_external: bool,
    #[serde(rename = "isFollowing", skip_serializing_if = "Option::is_none")]
    pub is_following: Option<bool>,
    #[serde(rename = "isFavorited", skip_serializing_if = "Option::is_none")]
    pub is_favorited: Option<bool>,
    #[serde(rename = "pinnedNotes", skip_serializing_if = "Option::is_none")]
    pub pinned_notes: Option<Vec<models::Note>>,
}

impl Channel {
    pub fn new(id: String, created_at: String, last_noted_at: Option<String>, name: String, description: Option<String>, user_id: Option<String>, banner_url: Option<String>, pinned_note_ids: Vec<String>, color: String, is_archived: bool, users_count: f64, notes_count: f64, is_sensitive: bool, allow_renote_to_external: bool) -> Channel {
        Channel {
            id,
            created_at,
            last_noted_at,
            name,
            description,
            user_id,
            banner_url,
            pinned_note_ids,
            color,
            is_archived,
            users_count,
            notes_count,
            is_sensitive,
            allow_renote_to_external,
            is_following: None,
            is_favorited: None,
            pinned_notes: None,
        }
    }
}
