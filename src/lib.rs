pub mod utils;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(PartialEq, Serialize, Deserialize, Clone, Default, Debug)]
pub struct RadioStation {
    pub changeuuid: String,
    pub stationuuid: String,
    pub serveruuid: Option<String>,
    pub name: String,
    pub url: String,
    pub url_resolved: String,
    pub homepage: String,
    pub favicon: String,
    pub tags: String,
    pub country: String,
    pub countrycode: String,
    pub iso_3166_2: Option<String>,
    pub state: String,
    pub language: String,
    pub languagecodes: Option<String>,
    pub votes: i32,
    //#[cfg(feature = "chrono")]
    //pub lastchangetime_iso8601: Option<DateTime<Utc>>,
    //#[cfg(not(feature = "chrono"))]
    pub lastchangetime_iso8601: Option<String>,
    pub codec: String,
    pub bitrate: u32,
    pub hls: i8,
    pub lastcheckok: i8,
    pub lastchecktime_iso8601: Option<DateTime<Utc>>,
    pub lastcheckoktime_iso8601: Option<DateTime<Utc>>,
    pub lastlocalchecktime_iso8601: Option<DateTime<Utc>>,
    pub clicktimestamp_iso8601: Option<DateTime<Utc>>,
    //#[cfg(not(feature = "chrono"))]
    //pub lastchecktime_iso8601: Option<String>,
    //#[cfg(not(feature = "chrono"))]
    //pub lastcheckoktime_iso8601: Option<String>,
    //#[cfg(not(feature = "chrono"))]
    //pub lastlocalchecktime_iso8601: Option<String>,
    //#[cfg(not(feature = "chrono"))]
    //pub clicktimestamp_iso8601: Option<String>,
    pub clickcount: u32,
    pub clicktrend: i32,
    pub ssl_error: Option<u8>,
    pub geo_lat: Option<f64>,
    pub geo_long: Option<f64>,
    pub has_extended_info: Option<bool>,
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Default, Debug)]
pub struct SearchData {
    pub name: String,
    pub country: String,
    pub language: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Client {
    Search { search_data: Arc<SearchData> },
    Play { url: Arc<String> },
    Stop,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Server {
    RadioChList { radio_list: Arc<Vec<RadioStation>> },
    Error(String),
}
