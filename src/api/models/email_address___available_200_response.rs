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
pub struct EmailAddressAvailable200Response {
    #[serde(rename = "available")]
    pub available: bool,
    #[serde(rename = "reason", deserialize_with = "Option::deserialize")]
    pub reason: Option<String>,
}

impl EmailAddressAvailable200Response {
    pub fn new(available: bool, reason: Option<String>) -> EmailAddressAvailable200Response {
        EmailAddressAvailable200Response {
            available,
            reason,
        }
    }
}

