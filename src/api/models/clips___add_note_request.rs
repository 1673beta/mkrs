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
pub struct ClipsAddNoteRequest {
    #[serde(rename = "clipId")]
    pub clip_id: String,
    #[serde(rename = "noteId")]
    pub note_id: String,
}

impl ClipsAddNoteRequest {
    pub fn new(clip_id: String, note_id: String) -> ClipsAddNoteRequest {
        ClipsAddNoteRequest {
            clip_id,
            note_id,
        }
    }
}

