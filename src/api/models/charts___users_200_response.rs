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
pub struct ChartsUsers200Response {
    #[serde(rename = "local")]
    pub local: Box<models::ChartsInstance200ResponseUsers>,
    #[serde(rename = "remote")]
    pub remote: Box<models::ChartsInstance200ResponseUsers>,
}

impl ChartsUsers200Response {
    pub fn new(local: models::ChartsInstance200ResponseUsers, remote: models::ChartsInstance200ResponseUsers) -> ChartsUsers200Response {
        ChartsUsers200Response {
            local: Box::new(local),
            remote: Box::new(remote),
        }
    }
}

