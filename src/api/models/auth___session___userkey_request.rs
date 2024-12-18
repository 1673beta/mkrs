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
pub struct AuthSessionUserkeyRequest {
    #[serde(rename = "appSecret")]
    pub app_secret: String,
    #[serde(rename = "token")]
    pub token: String,
}

impl AuthSessionUserkeyRequest {
    pub fn new(app_secret: String, token: String) -> AuthSessionUserkeyRequest {
        AuthSessionUserkeyRequest {
            app_secret,
            token,
        }
    }
}
