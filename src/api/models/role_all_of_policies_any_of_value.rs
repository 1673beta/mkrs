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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleAllOfPoliciesAnyOfValue {
    Integer(i32),
    Boolean(bool),
}

impl Default for RoleAllOfPoliciesAnyOfValue {
    fn default() -> Self {
        Self::Integer(Default::default())
    }
}

