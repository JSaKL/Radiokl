use super::Seeker;
use radioklw::utils::RadioError;
use radioklw::RadioStation;
use std::{collections::HashMap, fmt::Display};

#[allow(dead_code)]
pub enum StationOrder {
    Name,
    Url,
    Homepage,
    Favicon,
    Tags,
    Country,
    State,
    Language,
    Votes,
    Codec,
    Bitrate,
    Lastcheckok,
    Lastchecktime,
    Clicktimestamp,
    Clickcount,
    Clicktrend,
    Changetimestamp,
    Random,
}

impl Display for StationOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            StationOrder::Name => write!(f, "name"),
            StationOrder::Url => write!(f, "url"),
            StationOrder::Homepage => write!(f, "homepage"),
            StationOrder::Favicon => write!(f, "favicon"),
            StationOrder::Tags => write!(f, "tags"),
            StationOrder::Country => write!(f, "country"),
            StationOrder::State => write!(f, "state"),
            StationOrder::Language => write!(f, "language"),
            StationOrder::Votes => write!(f, "votes"),
            StationOrder::Codec => write!(f, "codec"),
            StationOrder::Bitrate => write!(f, "bitrate"),
            StationOrder::Lastcheckok => write!(f, "lastcheckok"),
            StationOrder::Lastchecktime => write!(f, "lastchecktime"),
            StationOrder::Clicktimestamp => write!(f, "clicktimestamp"),
            StationOrder::Clickcount => write!(f, "clickcount"),
            StationOrder::Clicktrend => write!(f, "clicktrend"),
            StationOrder::Changetimestamp => write!(f, "changetimestamp"),
            StationOrder::Random => write!(f, "random"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SearchMapper {
    map: HashMap<String, String>,
    api: Seeker,
}

impl SearchMapper {
    pub fn new(api: Seeker) -> Self {
        SearchMapper {
            api,
            map: HashMap::new(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.map.insert(String::from("name"), name);
        self
    }

    pub fn country(mut self, country: String) -> Self {
        self.map.insert(String::from("country"), country);
        self
    }

    pub fn language(mut self, language: String) -> Self {
        self.map.insert(String::from("language"), language);
        self
    }

    pub fn reverse(mut self, reverse: bool) -> Self {
        self.map
            .insert(String::from("reverse"), reverse.to_string());
        self
    }

    pub fn order(mut self, order: StationOrder) -> Self {
        self.map.insert(String::from("order"), order.to_string());
        self
    }

    pub async fn send(self) -> Result<Vec<RadioStation>, RadioError> {
        self.api.send("/json/stations/search", &self.map).await
    }
}
