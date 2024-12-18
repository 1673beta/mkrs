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
pub struct ChartsNotes200Response {
    #[serde(rename = "local")]
    pub local: Box<models::ChartsInstance200ResponseNotes>,
    #[serde(rename = "remote")]
    pub remote: Box<models::ChartsInstance200ResponseNotes>,
}

impl ChartsNotes200Response {
    pub fn new(local: models::ChartsInstance200ResponseNotes, remote: models::ChartsInstance200ResponseNotes) -> ChartsNotes200Response {
        ChartsNotes200Response {
            local: Box::new(local),
            remote: Box::new(remote),
        }
    }
}

