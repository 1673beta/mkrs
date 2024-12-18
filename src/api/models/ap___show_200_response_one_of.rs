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
pub struct ApShow200ResponseOneOf {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "object")]
    pub object: models::UserDetailedNotMe,
}

impl ApShow200ResponseOneOf {
    pub fn new(r#type: Type, object: models::UserDetailedNotMe) -> ApShow200ResponseOneOf {
        ApShow200ResponseOneOf {
            r#type,
            object,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "User")]
    User,
}

impl Default for Type {
    fn default() -> Type {
        Self::User
    }
}

