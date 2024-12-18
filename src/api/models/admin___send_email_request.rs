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
pub struct AdminSendEmailRequest {
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(rename = "text")]
    pub text: String,
}

impl AdminSendEmailRequest {
    pub fn new(to: String, subject: String, text: String) -> AdminSendEmailRequest {
        AdminSendEmailRequest {
            to,
            subject,
            text,
        }
    }
}
