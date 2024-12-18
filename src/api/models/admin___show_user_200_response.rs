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
pub struct AdminShowUser200Response {
    #[serde(rename = "email", deserialize_with = "Option::deserialize")]
    pub email: Option<String>,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(rename = "autoAcceptFollowed")]
    pub auto_accept_followed: bool,
    #[serde(rename = "noCrawle")]
    pub no_crawle: bool,
    #[serde(rename = "preventAiLearning")]
    pub prevent_ai_learning: bool,
    #[serde(rename = "alwaysMarkNsfw")]
    pub always_mark_nsfw: bool,
    #[serde(rename = "autoSensitive")]
    pub auto_sensitive: bool,
    #[serde(rename = "carefulBot")]
    pub careful_bot: bool,
    #[serde(rename = "injectFeaturedNote")]
    pub inject_featured_note: bool,
    #[serde(rename = "receiveAnnouncementEmail")]
    pub receive_announcement_email: bool,
    #[serde(rename = "mutedWords")]
    pub muted_words: Vec<models::AdminShowUser200ResponseMutedWordsInner>,
    #[serde(rename = "mutedInstances")]
    pub muted_instances: Vec<String>,
    #[serde(rename = "notificationRecieveConfig")]
    pub notification_recieve_config: Box<models::AdminShowUser200ResponseNotificationRecieveConfig>,
    #[serde(rename = "isModerator")]
    pub is_moderator: bool,
    #[serde(rename = "isSilenced")]
    pub is_silenced: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
    #[serde(rename = "isHibernated")]
    pub is_hibernated: bool,
    #[serde(rename = "lastActiveDate", deserialize_with = "Option::deserialize")]
    pub last_active_date: Option<String>,
    #[serde(rename = "moderationNote")]
    pub moderation_note: String,
    #[serde(rename = "signins")]
    pub signins: Vec<models::Signin>,
    #[serde(rename = "policies")]
    pub policies: models::RolePolicies,
    #[serde(rename = "roles")]
    pub roles: Vec<models::Role>,
    #[serde(rename = "roleAssigns")]
    pub role_assigns: Vec<models::AdminShowUser200ResponseRoleAssignsInner>,
}

impl AdminShowUser200Response {
    pub fn new(email: Option<String>, email_verified: bool, auto_accept_followed: bool, no_crawle: bool, prevent_ai_learning: bool, always_mark_nsfw: bool, auto_sensitive: bool, careful_bot: bool, inject_featured_note: bool, receive_announcement_email: bool, muted_words: Vec<models::AdminShowUser200ResponseMutedWordsInner>, muted_instances: Vec<String>, notification_recieve_config: models::AdminShowUser200ResponseNotificationRecieveConfig, is_moderator: bool, is_silenced: bool, is_suspended: bool, is_hibernated: bool, last_active_date: Option<String>, moderation_note: String, signins: Vec<models::Signin>, policies: models::RolePolicies, roles: Vec<models::Role>, role_assigns: Vec<models::AdminShowUser200ResponseRoleAssignsInner>) -> AdminShowUser200Response {
        AdminShowUser200Response {
            email,
            email_verified,
            auto_accept_followed,
            no_crawle,
            prevent_ai_learning,
            always_mark_nsfw,
            auto_sensitive,
            careful_bot,
            inject_featured_note,
            receive_announcement_email,
            muted_words,
            muted_instances,
            notification_recieve_config: Box::new(notification_recieve_config),
            is_moderator,
            is_silenced,
            is_suspended,
            is_hibernated,
            last_active_date,
            moderation_note,
            signins,
            policies,
            roles,
            role_assigns,
        }
    }
}

