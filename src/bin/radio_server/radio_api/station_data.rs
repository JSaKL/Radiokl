#[derive(Default, Debug, Clone, PartialEq, serde_derive::Deserialize)]
pub struct ServerStats {
    pub supported_version: i64,
    pub software_version: String,
    pub status: String,
    pub stations: i64,
    pub stations_broken: i64,
    pub tags: i64,
    pub clicks_last_hour: i64,
    pub clicks_last_day: i64,
    pub languages: i64,
    pub countries: i64,
}
