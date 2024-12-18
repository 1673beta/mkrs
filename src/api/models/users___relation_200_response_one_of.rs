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
pub struct UsersRelation200ResponseOneOf {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isFollowing")]
    pub is_following: bool,
    #[serde(rename = "hasPendingFollowRequestFromYou")]
    pub has_pending_follow_request_from_you: bool,
    #[serde(rename = "hasPendingFollowRequestToYou")]
    pub has_pending_follow_request_to_you: bool,
    #[serde(rename = "isFollowed")]
    pub is_followed: bool,
    #[serde(rename = "isBlocking")]
    pub is_blocking: bool,
    #[serde(rename = "isBlocked")]
    pub is_blocked: bool,
    #[serde(rename = "isMuted")]
    pub is_muted: bool,
    #[serde(rename = "isRenoteMuted")]
    pub is_renote_muted: bool,
}

impl UsersRelation200ResponseOneOf {
    pub fn new(id: String, is_following: bool, has_pending_follow_request_from_you: bool, has_pending_follow_request_to_you: bool, is_followed: bool, is_blocking: bool, is_blocked: bool, is_muted: bool, is_renote_muted: bool) -> UsersRelation200ResponseOneOf {
        UsersRelation200ResponseOneOf {
            id,
            is_following,
            has_pending_follow_request_from_you,
            has_pending_follow_request_to_you,
            is_followed,
            is_blocking,
            is_blocked,
            is_muted,
            is_renote_muted,
        }
    }
}

