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
pub struct MeDetailedOnly {
    #[serde(rename = "avatarId", deserialize_with = "Option::deserialize")]
    pub avatar_id: Option<String>,
    #[serde(rename = "bannerId", deserialize_with = "Option::deserialize")]
    pub banner_id: Option<String>,
    #[serde(rename = "isModerator", deserialize_with = "Option::deserialize")]
    pub is_moderator: Option<bool>,
    #[serde(rename = "isAdmin", deserialize_with = "Option::deserialize")]
    pub is_admin: Option<bool>,
    #[serde(rename = "injectFeaturedNote")]
    pub inject_featured_note: bool,
    #[serde(rename = "receiveAnnouncementEmail")]
    pub receive_announcement_email: bool,
    #[serde(rename = "alwaysMarkNsfw")]
    pub always_mark_nsfw: bool,
    #[serde(rename = "autoSensitive")]
    pub auto_sensitive: bool,
    #[serde(rename = "carefulBot")]
    pub careful_bot: bool,
    #[serde(rename = "autoAcceptFollowed")]
    pub auto_accept_followed: bool,
    #[serde(rename = "noCrawle")]
    pub no_crawle: bool,
    #[serde(rename = "preventAiLearning")]
    pub prevent_ai_learning: bool,
    #[serde(rename = "isExplorable")]
    pub is_explorable: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "twoFactorBackupCodesStock")]
    pub two_factor_backup_codes_stock: TwoFactorBackupCodesStock,
    #[serde(rename = "hideOnlineStatus")]
    pub hide_online_status: bool,
    #[serde(rename = "hasUnreadSpecifiedNotes")]
    pub has_unread_specified_notes: bool,
    #[serde(rename = "hasUnreadMentions")]
    pub has_unread_mentions: bool,
    #[serde(rename = "hasUnreadAnnouncement")]
    pub has_unread_announcement: bool,
    #[serde(rename = "unreadAnnouncements")]
    pub unread_announcements: Vec<models::Announcement>,
    #[serde(rename = "hasUnreadAntenna")]
    pub has_unread_antenna: bool,
    #[serde(rename = "hasUnreadChannel")]
    pub has_unread_channel: bool,
    #[serde(rename = "hasUnreadNotification")]
    pub has_unread_notification: bool,
    #[serde(rename = "hasPendingReceivedFollowRequest")]
    pub has_pending_received_follow_request: bool,
    #[serde(rename = "unreadNotificationsCount")]
    pub unread_notifications_count: f64,
    #[serde(rename = "mutedWords")]
    pub muted_words: Vec<Vec<String>>,
    #[serde(rename = "hardMutedWords")]
    pub hard_muted_words: Vec<Vec<String>>,
    #[serde(rename = "mutedInstances", deserialize_with = "Option::deserialize")]
    pub muted_instances: Option<Vec<String>>,
    #[serde(rename = "notificationRecieveConfig")]
    pub notification_recieve_config: Box<models::AdminShowUser200ResponseNotificationRecieveConfig>,
    #[serde(rename = "emailNotificationTypes")]
    pub email_notification_types: Vec<String>,
    #[serde(rename = "achievements")]
    pub achievements: Vec<models::UsersAchievements200ResponseInner>,
    #[serde(rename = "loggedInDays")]
    pub logged_in_days: f64,
    #[serde(rename = "policies")]
    pub policies: models::RolePolicies,
    #[serde(rename = "email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    #[serde(rename = "emailVerified", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<Option<bool>>,
    #[serde(rename = "securityKeysList", skip_serializing_if = "Option::is_none")]
    pub security_keys_list: Option<Vec<models::MeDetailedOnlySecurityKeysListInner>>,
}

impl MeDetailedOnly {
    pub fn new(avatar_id: Option<String>, banner_id: Option<String>, is_moderator: Option<bool>, is_admin: Option<bool>, inject_featured_note: bool, receive_announcement_email: bool, always_mark_nsfw: bool, auto_sensitive: bool, careful_bot: bool, auto_accept_followed: bool, no_crawle: bool, prevent_ai_learning: bool, is_explorable: bool, is_deleted: bool, two_factor_backup_codes_stock: TwoFactorBackupCodesStock, hide_online_status: bool, has_unread_specified_notes: bool, has_unread_mentions: bool, has_unread_announcement: bool, unread_announcements: Vec<models::Announcement>, has_unread_antenna: bool, has_unread_channel: bool, has_unread_notification: bool, has_pending_received_follow_request: bool, unread_notifications_count: f64, muted_words: Vec<Vec<String>>, hard_muted_words: Vec<Vec<String>>, muted_instances: Option<Vec<String>>, notification_recieve_config: models::AdminShowUser200ResponseNotificationRecieveConfig, email_notification_types: Vec<String>, achievements: Vec<models::UsersAchievements200ResponseInner>, logged_in_days: f64, policies: models::RolePolicies) -> MeDetailedOnly {
        MeDetailedOnly {
            avatar_id,
            banner_id,
            is_moderator,
            is_admin,
            inject_featured_note,
            receive_announcement_email,
            always_mark_nsfw,
            auto_sensitive,
            careful_bot,
            auto_accept_followed,
            no_crawle,
            prevent_ai_learning,
            is_explorable,
            is_deleted,
            two_factor_backup_codes_stock,
            hide_online_status,
            has_unread_specified_notes,
            has_unread_mentions,
            has_unread_announcement,
            unread_announcements,
            has_unread_antenna,
            has_unread_channel,
            has_unread_notification,
            has_pending_received_follow_request,
            unread_notifications_count,
            muted_words,
            hard_muted_words,
            muted_instances,
            notification_recieve_config: Box::new(notification_recieve_config),
            email_notification_types,
            achievements,
            logged_in_days,
            policies,
            email: None,
            email_verified: None,
            security_keys_list: None,
        }
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

