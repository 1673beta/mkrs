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
pub struct DriveFoldersUpdateRequest {
    #[serde(rename = "folderId")]
    pub folder_id: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "parentId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Option<String>>,
}

impl DriveFoldersUpdateRequest {
    pub fn new(folder_id: String) -> DriveFoldersUpdateRequest {
        DriveFoldersUpdateRequest {
            folder_id,
            name: None,
            parent_id: None,
        }
    }
}

