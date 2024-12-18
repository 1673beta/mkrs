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
pub struct NotesCreateRequest {
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    #[serde(rename = "visibleUserIds", skip_serializing_if = "Option::is_none")]
    pub visible_user_ids: Option<Vec<String>>,
    #[serde(rename = "cw", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cw: Option<Option<String>>,
    #[serde(rename = "localOnly", skip_serializing_if = "Option::is_none")]
    pub local_only: Option<bool>,
    #[serde(rename = "reactionAcceptance", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reaction_acceptance: Option<Option<ReactionAcceptance>>,
    #[serde(rename = "noExtractMentions", skip_serializing_if = "Option::is_none")]
    pub no_extract_mentions: Option<bool>,
    #[serde(rename = "noExtractHashtags", skip_serializing_if = "Option::is_none")]
    pub no_extract_hashtags: Option<bool>,
    #[serde(rename = "noExtractEmojis", skip_serializing_if = "Option::is_none")]
    pub no_extract_emojis: Option<bool>,
    #[serde(rename = "replyId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reply_id: Option<Option<String>>,
    #[serde(rename = "renoteId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub renote_id: Option<Option<String>>,
    #[serde(rename = "channelId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Option<String>>,
    #[serde(rename = "text", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub text: Option<Option<String>>,
    #[serde(rename = "fileIds", skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    #[serde(rename = "mediaIds", skip_serializing_if = "Option::is_none")]
    pub media_ids: Option<Vec<String>>,
    #[serde(rename = "poll", skip_serializing_if = "Option::is_none")]
    pub poll: Option<Box<models::NotesCreateRequestPoll>>,
}

impl NotesCreateRequest {
    pub fn new() -> NotesCreateRequest {
        NotesCreateRequest {
            visibility: None,
            visible_user_ids: None,
            cw: None,
            local_only: None,
            reaction_acceptance: None,
            no_extract_mentions: None,
            no_extract_hashtags: None,
            no_extract_emojis: None,
            reply_id: None,
            renote_id: None,
            channel_id: None,
            text: None,
            file_ids: None,
            media_ids: None,
            poll: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "followers")]
    Followers,
    #[serde(rename = "specified")]
    Specified,
}

impl Default for Visibility {
    fn default() -> Visibility {
        Self::Public
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReactionAcceptance {
    #[serde(rename = "likeOnly")]
    LikeOnly,
    #[serde(rename = "likeOnlyForRemote")]
    LikeOnlyForRemote,
    #[serde(rename = "nonSensitiveOnly")]
    NonSensitiveOnly,
    #[serde(rename = "nonSensitiveOnlyForLocalLikeOnlyForRemote")]
    NonSensitiveOnlyForLocalLikeOnlyForRemote,
}

impl Default for ReactionAcceptance {
    fn default() -> ReactionAcceptance {
        Self::LikeOnly
    }
}

