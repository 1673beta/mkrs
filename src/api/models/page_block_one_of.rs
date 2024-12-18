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
pub struct PageBlockOneOf {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "text")]
    pub text: String,
}

impl PageBlockOneOf {
    pub fn new(id: String, r#type: Type, text: String) -> PageBlockOneOf {
        PageBlockOneOf {
            id,
            r#type,
            text,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "text")]
    Text,
}

impl Default for Type {
    fn default() -> Type {
        Self::Text
    }
}

