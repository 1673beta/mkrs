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
pub struct AdminRolesUnassignRequest {
    #[serde(rename = "roleId")]
    pub role_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl AdminRolesUnassignRequest {
    pub fn new(role_id: String, user_id: String) -> AdminRolesUnassignRequest {
        AdminRolesUnassignRequest {
            role_id,
            user_id,
        }
    }
}

