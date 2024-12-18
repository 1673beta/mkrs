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
pub struct AdminFederationDeleteAllFilesRequest {
    #[serde(rename = "host")]
    pub host: String,
}

impl AdminFederationDeleteAllFilesRequest {
    pub fn new(host: String) -> AdminFederationDeleteAllFilesRequest {
        AdminFederationDeleteAllFilesRequest {
            host,
        }
    }
}

