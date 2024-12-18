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
pub struct NotesHybridTimelineRequest {
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "sinceId", skip_serializing_if = "Option::is_none")]
    pub since_id: Option<String>,
    #[serde(rename = "untilId", skip_serializing_if = "Option::is_none")]
    pub until_id: Option<String>,
    #[serde(rename = "sinceDate", skip_serializing_if = "Option::is_none")]
    pub since_date: Option<i32>,
    #[serde(rename = "untilDate", skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i32>,
    #[serde(rename = "allowPartial", skip_serializing_if = "Option::is_none")]
    pub allow_partial: Option<bool>,
    #[serde(rename = "includeMyRenotes", skip_serializing_if = "Option::is_none")]
    pub include_my_renotes: Option<bool>,
    #[serde(rename = "includeRenotedMyNotes", skip_serializing_if = "Option::is_none")]
    pub include_renoted_my_notes: Option<bool>,
    #[serde(rename = "includeLocalRenotes", skip_serializing_if = "Option::is_none")]
    pub include_local_renotes: Option<bool>,
    #[serde(rename = "withFiles", skip_serializing_if = "Option::is_none")]
    pub with_files: Option<bool>,
    #[serde(rename = "withRenotes", skip_serializing_if = "Option::is_none")]
    pub with_renotes: Option<bool>,
    #[serde(rename = "withReplies", skip_serializing_if = "Option::is_none")]
    pub with_replies: Option<bool>,
}

impl NotesHybridTimelineRequest {
    pub fn new() -> NotesHybridTimelineRequest {
        NotesHybridTimelineRequest {
            limit: None,
            since_id: None,
            until_id: None,
            since_date: None,
            until_date: None,
            allow_partial: None,
            include_my_renotes: None,
            include_renoted_my_notes: None,
            include_local_renotes: None,
            with_files: None,
            with_renotes: None,
            with_replies: None,
        }
    }
}

