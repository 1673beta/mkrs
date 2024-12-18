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
pub struct NotesState200Response {
    #[serde(rename = "isFavorited")]
    pub is_favorited: bool,
    #[serde(rename = "isMutedThread")]
    pub is_muted_thread: bool,
}

impl NotesState200Response {
    pub fn new(is_favorited: bool, is_muted_thread: bool) -> NotesState200Response {
        NotesState200Response {
            is_favorited,
            is_muted_thread,
        }
    }
}

