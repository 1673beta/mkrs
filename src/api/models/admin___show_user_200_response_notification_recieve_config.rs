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
pub struct AdminShowUser200ResponseNotificationRecieveConfig {
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<Box<models::AdminShowUser200ResponseNotificationRecieveConfigNote>>,
    #[serde(rename = "follow", skip_serializing_if = "Option::is_none")]
    pub follow: Option<Box<models::AdminShowUser200ResponseNotificationRecieveConfigNote>>,
    #[serde(rename = "mention", skip_serializing_if = "Option::is_none")]
    pub mention: Option<Box<models::AdminShowUser200ResponseNotificationRecieveConfigNote>>,
    #[serde(rename = "reply", skip_serializing_if = "Option::is_none")]
    pub reply: Option<Box<models::AdminShowUser200ResponseNotificationRecieveConfigNote>>,
    #[serde(rename = "renote", skip_serializing_if = "Option::is_none")]
    pub renote: Option<Box<models::AdminShowUser200ResponseNotificationRecieveConfigNote>>,
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<models::AdminShowUser200ResponseNotificationRecieveConfigNote>>,
    #[serde(rename = "reaction", skip_serializing_if = "Option::is_none")]
    pub reaction: Option<Box<models::AdminShowUser200ResponseNotificationRecieveConfigNote>>,
    #[serde(rename = "pollEnded", skip_serializing_if = "Option::is_none")]
    pub poll_ended: Option<Box<models::AdminShowUser200ResponseNotificationRecieveConfigNote>>,
    #[serde(rename = "receiveFollowRequest", skip_serializing_if = "Option::is_none")]
    pub receive_follow_request: Option<Box<models::AdminShowUser200ResponseNotificationRecieveConfigNote>>,
    #[serde(rename = "followRequestAccepted", skip_serializing_if = "Option::is_none")]
    pub follow_request_accepted: Option<Box<models::AdminShowUser200ResponseNotificationRecieveConfigNote>>,
    #[serde(rename = "roleAssigned", skip_serializing_if = "Option::is_none")]
    pub role_assigned: Option<Box<models::AdminShowUser200ResponseNotificationRecieveConfigNote>>,
    #[serde(rename = "achievementEarned", skip_serializing_if = "Option::is_none")]
    pub achievement_earned: Option<Box<models::AdminShowUser200ResponseNotificationRecieveConfigNote>>,
    #[serde(rename = "app", skip_serializing_if = "Option::is_none")]
    pub app: Option<Box<models::AdminShowUser200ResponseNotificationRecieveConfigNote>>,
    #[serde(rename = "test", skip_serializing_if = "Option::is_none")]
    pub test: Option<Box<models::AdminShowUser200ResponseNotificationRecieveConfigNote>>,
}

impl AdminShowUser200ResponseNotificationRecieveConfig {
    pub fn new() -> AdminShowUser200ResponseNotificationRecieveConfig {
        AdminShowUser200ResponseNotificationRecieveConfig {
            note: None,
            follow: None,
            mention: None,
            reply: None,
            renote: None,
            quote: None,
            reaction: None,
            poll_ended: None,
            receive_follow_request: None,
            follow_request_accepted: None,
            role_assigned: None,
            achievement_earned: None,
            app: None,
            test: None,
        }
    }
}
