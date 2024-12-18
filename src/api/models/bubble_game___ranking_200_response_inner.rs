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
pub struct BubbleGameRanking200ResponseInner {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "score")]
    pub score: i32,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::UserLite>>,
}

impl BubbleGameRanking200ResponseInner {
    pub fn new(id: String, score: i32) -> BubbleGameRanking200ResponseInner {
        BubbleGameRanking200ResponseInner {
            id,
            score,
            user: None,
        }
    }
}

