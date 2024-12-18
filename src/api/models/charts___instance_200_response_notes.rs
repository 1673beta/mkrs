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
pub struct ChartsInstance200ResponseNotes {
    #[serde(rename = "total")]
    pub total: Vec<f64>,
    #[serde(rename = "inc")]
    pub inc: Vec<f64>,
    #[serde(rename = "dec")]
    pub dec: Vec<f64>,
    #[serde(rename = "diffs")]
    pub diffs: Box<models::ChartsInstance200ResponseNotesDiffs>,
}

impl ChartsInstance200ResponseNotes {
    pub fn new(total: Vec<f64>, inc: Vec<f64>, dec: Vec<f64>, diffs: models::ChartsInstance200ResponseNotesDiffs) -> ChartsInstance200ResponseNotes {
        ChartsInstance200ResponseNotes {
            total,
            inc,
            dec,
            diffs: Box::new(diffs),
        }
    }
}
