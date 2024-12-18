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
pub struct Blocking {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "blockeeId")]
    pub blockee_id: String,
    #[serde(rename = "blockee")]
    pub blockee: models::UserDetailedNotMe,
}

impl Blocking {
    pub fn new(id: String, created_at: String, blockee_id: String, blockee: models::UserDetailedNotMe) -> Blocking {
        Blocking {
            id,
            created_at,
            blockee_id,
            blockee,
        }
    }
}

