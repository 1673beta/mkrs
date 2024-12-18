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
pub struct NotesReactionsCreateRequest {
    #[serde(rename = "noteId")]
    pub note_id: String,
    #[serde(rename = "reaction")]
    pub reaction: String,
}

impl NotesReactionsCreateRequest {
    pub fn new(note_id: String, reaction: String) -> NotesReactionsCreateRequest {
        NotesReactionsCreateRequest {
            note_id,
            reaction,
        }
    }
}

