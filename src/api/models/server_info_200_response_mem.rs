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
pub struct ServerInfo200ResponseMem {
    #[serde(rename = "total")]
    pub total: f64,
}

impl ServerInfo200ResponseMem {
    pub fn new(total: f64) -> ServerInfo200ResponseMem {
        ServerInfo200ResponseMem {
            total,
        }
    }
}

