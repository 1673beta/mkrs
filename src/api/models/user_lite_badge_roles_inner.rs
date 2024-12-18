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
pub struct UserLiteBadgeRolesInner {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "iconUrl", deserialize_with = "Option::deserialize")]
    pub icon_url: Option<String>,
    #[serde(rename = "displayOrder")]
    pub display_order: f64,
}

impl UserLiteBadgeRolesInner {
    pub fn new(name: String, icon_url: Option<String>, display_order: f64) -> UserLiteBadgeRolesInner {
        UserLiteBadgeRolesInner {
            name,
            icon_url,
            display_order,
        }
    }
}

