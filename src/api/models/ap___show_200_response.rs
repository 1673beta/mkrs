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
pub enum ApShow200Response {
    ApShow200ResponseOneOf(Box<models::ApShow200ResponseOneOf>),
    ApShow200ResponseOneOf1(Box<models::ApShow200ResponseOneOf1>),
}

impl Default for ApShow200Response {
    fn default() -> Self {
        Self::ApShow200ResponseOneOf(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "User")]
    User,
    #[serde(rename = "Note")]
    Note,
}

impl Default for Type {
    fn default() -> Type {
        Self::User
    }
}
