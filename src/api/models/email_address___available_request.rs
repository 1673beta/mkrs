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
pub struct EmailAddressAvailableRequest {
    #[serde(rename = "emailAddress")]
    pub email_address: String,
}

impl EmailAddressAvailableRequest {
    pub fn new(email_address: String) -> EmailAddressAvailableRequest {
        EmailAddressAvailableRequest {
            email_address,
        }
    }
}

