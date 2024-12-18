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
pub struct GetOnlineUsersCount200Response {
    #[serde(rename = "count")]
    pub count: f64,
}

impl GetOnlineUsersCount200Response {
    pub fn new(count: f64) -> GetOnlineUsersCount200Response {
        GetOnlineUsersCount200Response {
            count,
        }
    }
}

