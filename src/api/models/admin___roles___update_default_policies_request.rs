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
pub struct AdminRolesUpdateDefaultPoliciesRequest {
    #[serde(rename = "policies")]
    pub policies: serde_json::Value,
}

impl AdminRolesUpdateDefaultPoliciesRequest {
    pub fn new(policies: serde_json::Value) -> AdminRolesUpdateDefaultPoliciesRequest {
        AdminRolesUpdateDefaultPoliciesRequest {
            policies,
        }
    }
}

