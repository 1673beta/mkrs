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
pub struct RoleCondFormulaValueCreated {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "sec")]
    pub sec: f64,
}

impl RoleCondFormulaValueCreated {
    pub fn new(id: String, r#type: Type, sec: f64) -> RoleCondFormulaValueCreated {
        RoleCondFormulaValueCreated {
            id,
            r#type,
            sec,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "createdLessThan")]
    CreatedLessThan,
    #[serde(rename = "createdMoreThan")]
    CreatedMoreThan,
}

impl Default for Type {
    fn default() -> Type {
        Self::CreatedLessThan
    }
}
