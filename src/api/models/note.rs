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
pub struct Note {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "deletedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<Option<String>>,
    #[serde(rename = "text", deserialize_with = "Option::deserialize")]
    pub text: Option<String>,
    #[serde(rename = "cw", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cw: Option<Option<String>>,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "user")]
    pub user: models::UserLite,
    #[serde(rename = "replyId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reply_id: Option<Option<String>>,
    #[serde(rename = "renoteId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub renote_id: Option<Option<String>>,
    #[serde(rename = "reply", skip_serializing_if = "Option::is_none")]
    pub reply: Option<Box<models::Note>>,
    #[serde(rename = "renote", skip_serializing_if = "Option::is_none")]
    pub renote: Option<Box<models::Note>>,
    #[serde(rename = "isHidden", skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[serde(rename = "visibility")]
    pub visibility: Visibility,
    #[serde(rename = "mentions", skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,
    #[serde(rename = "visibleUserIds", skip_serializing_if = "Option::is_none")]
    pub visible_user_ids: Option<Vec<String>>,
    #[serde(rename = "fileIds", skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<models::DriveFile>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "poll", skip_serializing_if = "Option::is_none")]
    pub poll: Option<Box<models::NotePoll>>,
    #[serde(rename = "emojis", skip_serializing_if = "Option::is_none")]
    pub emojis: Option<std::collections::HashMap<String, models::NoteEmojisValue>>,
    #[serde(rename = "channelId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Option<String>>,
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<Box<models::NoteChannel>>,
    #[serde(rename = "localOnly", skip_serializing_if = "Option::is_none")]
    pub local_only: Option<bool>,
    #[serde(rename = "reactionAcceptance", deserialize_with = "Option::deserialize")]
    pub reaction_acceptance: Option<ReactionAcceptance>,
    #[serde(rename = "reactionEmojis")]
    pub reaction_emojis: std::collections::HashMap<String, models::NoteEmojisValue>,
    #[serde(rename = "reactions")]
    pub reactions: std::collections::HashMap<String, models::Retention200ResponseInnerDataValue>,
    #[serde(rename = "reactionCount")]
    pub reaction_count: f64,
    #[serde(rename = "renoteCount")]
    pub renote_count: f64,
    #[serde(rename = "repliesCount")]
    pub replies_count: f64,
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "reactionAndUserPairCache", skip_serializing_if = "Option::is_none")]
    pub reaction_and_user_pair_cache: Option<Vec<String>>,
    #[serde(rename = "clippedCount", skip_serializing_if = "Option::is_none")]
    pub clipped_count: Option<f64>,
    #[serde(rename = "myReaction", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub my_reaction: Option<Option<String>>,
}

impl Note {
    pub fn new(id: String, created_at: String, text: Option<String>, user_id: String, user: models::UserLite, visibility: Visibility, reaction_acceptance: Option<ReactionAcceptance>, reaction_emojis: std::collections::HashMap<String, models::NoteEmojisValue>, reactions: std::collections::HashMap<String, models::Retention200ResponseInnerDataValue>, reaction_count: f64, renote_count: f64, replies_count: f64) -> Note {
        Note {
            id,
            created_at,
            deleted_at: None,
            text,
            cw: None,
            user_id,
            user,
            reply_id: None,
            renote_id: None,
            reply: None,
            renote: None,
            is_hidden: None,
            visibility,
            mentions: None,
            visible_user_ids: None,
            file_ids: None,
            files: None,
            tags: None,
            poll: None,
            emojis: None,
            channel_id: None,
            channel: None,
            local_only: None,
            reaction_acceptance,
            reaction_emojis,
            reactions,
            reaction_count,
            renote_count,
            replies_count,
            uri: None,
            url: None,
            reaction_and_user_pair_cache: None,
            clipped_count: None,
            my_reaction: None,
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

