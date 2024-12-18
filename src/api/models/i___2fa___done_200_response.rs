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
pub struct I2faDone200Response {
    #[serde(rename = "backupCodes")]
    pub backup_codes: Vec<String>,
}

impl I2faDone200Response {
    pub fn new(backup_codes: Vec<String>) -> I2faDone200Response {
        I2faDone200Response {
            backup_codes,
        }
    }
}

