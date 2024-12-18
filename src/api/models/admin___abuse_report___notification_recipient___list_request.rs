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
pub struct AdminAbuseReportNotificationRecipientListRequest {
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<Vec<Method>>,
}

impl AdminAbuseReportNotificationRecipientListRequest {
    pub fn new() -> AdminAbuseReportNotificationRecipientListRequest {
        AdminAbuseReportNotificationRecipientListRequest {
            method: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "webhook")]
    Webhook,
}

impl Default for Method {
    fn default() -> Method {
        Self::Email
    }
}
