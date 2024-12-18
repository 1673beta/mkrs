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
pub struct ChartsDrive200ResponseLocal {
    #[serde(rename = "incCount")]
    pub inc_count: Vec<f64>,
    #[serde(rename = "incSize")]
    pub inc_size: Vec<f64>,
    #[serde(rename = "decCount")]
    pub dec_count: Vec<f64>,
    #[serde(rename = "decSize")]
    pub dec_size: Vec<f64>,
}

impl ChartsDrive200ResponseLocal {
    pub fn new(inc_count: Vec<f64>, inc_size: Vec<f64>, dec_count: Vec<f64>, dec_size: Vec<f64>) -> ChartsDrive200ResponseLocal {
        ChartsDrive200ResponseLocal {
            inc_count,
            inc_size,
            dec_count,
            dec_size,
        }
    }
}
