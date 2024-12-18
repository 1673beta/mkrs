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
pub struct AdminRolesCreateRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "color", deserialize_with = "Option::deserialize")]
    pub color: Option<String>,
    #[serde(rename = "iconUrl", deserialize_with = "Option::deserialize")]
    pub icon_url: Option<String>,
    #[serde(rename = "target")]
    pub target: Target,
    #[serde(rename = "condFormula")]
    pub cond_formula: serde_json::Value,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "isModerator")]
    pub is_moderator: bool,
    #[serde(rename = "isAdministrator")]
    pub is_administrator: bool,
    #[serde(rename = "isExplorable", skip_serializing_if = "Option::is_none")]
    pub is_explorable: Option<bool>,
    #[serde(rename = "asBadge")]
    pub as_badge: bool,
    #[serde(rename = "canEditMembersByModerator")]
    pub can_edit_members_by_moderator: bool,
    #[serde(rename = "displayOrder")]
    pub display_order: f64,
    #[serde(rename = "policies")]
    pub policies: serde_json::Value,
}

impl AdminRolesCreateRequest {
    pub fn new(name: String, description: String, color: Option<String>, icon_url: Option<String>, target: Target, cond_formula: serde_json::Value, is_public: bool, is_moderator: bool, is_administrator: bool, as_badge: bool, can_edit_members_by_moderator: bool, display_order: f64, policies: serde_json::Value) -> AdminRolesCreateRequest {
        AdminRolesCreateRequest {
            name,
            description,
            color,
            icon_url,
            target,
            cond_formula,
            is_public,
            is_moderator,
            is_administrator,
            is_explorable: None,
            as_badge,
            can_edit_members_by_moderator,
            display_order,
            policies,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Target {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "conditional")]
    Conditional,
}

impl Default for Target {
    fn default() -> Target {
        Self::Manual
    }
}
