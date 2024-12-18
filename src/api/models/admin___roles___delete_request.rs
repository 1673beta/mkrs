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
pub struct AdminRolesDeleteRequest {
    #[serde(rename = "roleId")]
    pub role_id: String,
}

impl AdminRolesDeleteRequest {
    pub fn new(role_id: String) -> AdminRolesDeleteRequest {
        AdminRolesDeleteRequest {
            role_id,
        }
    }
}
