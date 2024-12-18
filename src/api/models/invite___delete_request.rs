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
pub struct InviteDeleteRequest {
    #[serde(rename = "inviteId")]
    pub invite_id: String,
}

impl InviteDeleteRequest {
    pub fn new(invite_id: String) -> InviteDeleteRequest {
        InviteDeleteRequest {
            invite_id,
        }
    }
}
