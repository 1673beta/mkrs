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
pub struct Announcement {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt", deserialize_with = "Option::deserialize")]
    pub updated_at: Option<String>,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "imageUrl", deserialize_with = "Option::deserialize")]
    pub image_url: Option<String>,
    #[serde(rename = "icon")]
    pub icon: Icon,
    #[serde(rename = "display")]
    pub display: Display,
    #[serde(rename = "needConfirmationToRead")]
    pub need_confirmation_to_read: bool,
    #[serde(rename = "silence")]
    pub silence: bool,
    #[serde(rename = "forYou")]
    pub for_you: bool,
    #[serde(rename = "isRead", skip_serializing_if = "Option::is_none")]
    pub is_read: Option<bool>,
}

impl Announcement {
    pub fn new(id: String, created_at: String, updated_at: Option<String>, text: String, title: String, image_url: Option<String>, icon: Icon, display: Display, need_confirmation_to_read: bool, silence: bool, for_you: bool) -> Announcement {
        Announcement {
            id,
            created_at,
            updated_at,
            text,
            title,
            image_url,
            icon,
            display,
            need_confirmation_to_read,
            silence,
            for_you,
            is_read: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Icon {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "success")]
    Success,
}

impl Default for Icon {
    fn default() -> Icon {
        Self::Info
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Display {
    #[serde(rename = "dialog")]
    Dialog,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "banner")]
    Banner,
}

impl Default for Display {
    fn default() -> Display {
        Self::Dialog
    }
}
