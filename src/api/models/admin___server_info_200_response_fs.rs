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
pub struct AdminServerInfo200ResponseFs {
    #[serde(rename = "total")]
    pub total: f64,
    #[serde(rename = "used")]
    pub used: f64,
}

impl AdminServerInfo200ResponseFs {
    pub fn new(total: f64, used: f64) -> AdminServerInfo200ResponseFs {
        AdminServerInfo200ResponseFs {
            total,
            used,
        }
    }
}

