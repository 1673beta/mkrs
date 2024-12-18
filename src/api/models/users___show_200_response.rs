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
pub enum UsersShow200Response {
    UserDetailed(models::UserDetailed),
    Array(Vec<models::UserDetailed>),
}

impl Default for UsersShow200Response {
    fn default() -> Self {
        Self::UserDetailed(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnlineStatus {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "offline")]
    Offline,
}

impl Default for OnlineStatus {
    fn default() -> OnlineStatus {
        Self::Unknown
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FollowingVisibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "followers")]
    Followers,
    #[serde(rename = "private")]
    Private,
}

impl Default for FollowingVisibility {
    fn default() -> FollowingVisibility {
        Self::Public
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FollowersVisibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "followers")]
    Followers,
    #[serde(rename = "private")]
    Private,
}

impl Default for FollowersVisibility {
    fn default() -> FollowersVisibility {
        Self::Public
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Notify {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "none")]
    None,
}

impl Default for Notify {
    fn default() -> Notify {
        Self::Normal
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TwoFactorBackupCodesStock {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "none")]
    None,
}

impl Default for TwoFactorBackupCodesStock {
    fn default() -> TwoFactorBackupCodesStock {
        Self::Full
    }
}
