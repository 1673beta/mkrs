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
pub struct AdminAccountsCreateRequest {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "password")]
    pub password: String,
}

impl AdminAccountsCreateRequest {
    pub fn new(username: String, password: String) -> AdminAccountsCreateRequest {
        AdminAccountsCreateRequest {
            username,
            password,
        }
    }
}

