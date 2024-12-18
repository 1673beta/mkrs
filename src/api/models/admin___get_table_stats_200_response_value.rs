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
pub struct AdminGetTableStats200ResponseValue {
    #[serde(rename = "count")]
    pub count: f64,
    #[serde(rename = "size")]
    pub size: f64,
}

impl AdminGetTableStats200ResponseValue {
    pub fn new(count: f64, size: f64) -> AdminGetTableStats200ResponseValue {
        AdminGetTableStats200ResponseValue {
            count,
            size,
        }
    }
}
