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
pub struct SwRegisterRequest {
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    #[serde(rename = "auth")]
    pub auth: String,
    #[serde(rename = "publickey")]
    pub publickey: String,
    #[serde(rename = "sendReadMessage", skip_serializing_if = "Option::is_none")]
    pub send_read_message: Option<bool>,
}

impl SwRegisterRequest {
    pub fn new(endpoint: String, auth: String, publickey: String) -> SwRegisterRequest {
        SwRegisterRequest {
            endpoint,
            auth,
            publickey,
            send_read_message: None,
        }
    }
}

