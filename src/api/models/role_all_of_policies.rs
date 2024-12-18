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
pub struct RoleAllOfPolicies {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<models::RoleAllOfPoliciesAnyOfValue>>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "useDefault", skip_serializing_if = "Option::is_none")]
    pub use_default: Option<bool>,
}

impl RoleAllOfPolicies {
    pub fn new() -> RoleAllOfPolicies {
        RoleAllOfPolicies {
            value: None,
            priority: None,
            use_default: None,
        }
    }
}
