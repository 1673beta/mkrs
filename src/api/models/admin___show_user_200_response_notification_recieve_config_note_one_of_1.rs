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
pub struct AdminShowUser200ResponseNotificationRecieveConfigNoteOneOf1 {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "userListId")]
    pub user_list_id: String,
}

impl AdminShowUser200ResponseNotificationRecieveConfigNoteOneOf1 {
    pub fn new(r#type: Type, user_list_id: String) -> AdminShowUser200ResponseNotificationRecieveConfigNoteOneOf1 {
        AdminShowUser200ResponseNotificationRecieveConfigNoteOneOf1 {
            r#type,
            user_list_id,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "list")]
    List,
}

impl Default for Type {
    fn default() -> Type {
        Self::List
    }
}

