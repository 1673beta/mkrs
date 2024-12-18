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
pub struct AdminSystemWebhookTestRequest {
    #[serde(rename = "webhookId")]
    pub webhook_id: String,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    pub r#override: Option<Box<models::AdminSystemWebhookTestRequestOverride>>,
}

impl AdminSystemWebhookTestRequest {
    pub fn new(webhook_id: String, r#type: Type) -> AdminSystemWebhookTestRequest {
        AdminSystemWebhookTestRequest {
            webhook_id,
            r#type,
            r#override: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "abuseReport")]
    AbuseReport,
    #[serde(rename = "abuseReportResolved")]
    AbuseReportResolved,
    #[serde(rename = "userCreated")]
    UserCreated,
}

impl Default for Type {
    fn default() -> Type {
        Self::AbuseReport
    }
}
