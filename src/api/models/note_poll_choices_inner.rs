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
pub struct NotePollChoicesInner {
    #[serde(rename = "isVoted")]
    pub is_voted: bool,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "votes")]
    pub votes: f64,
}

impl NotePollChoicesInner {
    pub fn new(is_voted: bool, text: String, votes: f64) -> NotePollChoicesInner {
        NotePollChoicesInner {
            is_voted,
            text,
            votes,
        }
    }
}

