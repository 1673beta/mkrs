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
pub struct Stats200Response {
    #[serde(rename = "notesCount")]
    pub notes_count: f64,
    #[serde(rename = "originalNotesCount")]
    pub original_notes_count: f64,
    #[serde(rename = "usersCount")]
    pub users_count: f64,
    #[serde(rename = "originalUsersCount")]
    pub original_users_count: f64,
    #[serde(rename = "instances")]
    pub instances: f64,
    #[serde(rename = "driveUsageLocal")]
    pub drive_usage_local: f64,
    #[serde(rename = "driveUsageRemote")]
    pub drive_usage_remote: f64,
}

impl Stats200Response {
    pub fn new(notes_count: f64, original_notes_count: f64, users_count: f64, original_users_count: f64, instances: f64, drive_usage_local: f64, drive_usage_remote: f64) -> Stats200Response {
        Stats200Response {
            notes_count,
            original_notes_count,
            users_count,
            original_users_count,
            instances,
            drive_usage_local,
            drive_usage_remote,
        }
    }
}
