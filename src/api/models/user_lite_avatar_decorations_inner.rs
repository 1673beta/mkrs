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
pub struct UserLiteAvatarDecorationsInner {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "angle", skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    #[serde(rename = "flipH", skip_serializing_if = "Option::is_none")]
    pub flip_h: Option<bool>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "offsetX", skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<f64>,
    #[serde(rename = "offsetY", skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<f64>,
}

impl UserLiteAvatarDecorationsInner {
    pub fn new(id: String, url: String) -> UserLiteAvatarDecorationsInner {
        UserLiteAvatarDecorationsInner {
            id,
            angle: None,
            flip_h: None,
            url,
            offset_x: None,
            offset_y: None,
        }
    }
}

