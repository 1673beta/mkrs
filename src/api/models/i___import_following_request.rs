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
pub struct IImportFollowingRequest {
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[serde(rename = "withReplies", skip_serializing_if = "Option::is_none")]
    pub with_replies: Option<bool>,
}

impl IImportFollowingRequest {
    pub fn new(file_id: String) -> IImportFollowingRequest {
        IImportFollowingRequest {
            file_id,
            with_replies: None,
        }
    }
}

