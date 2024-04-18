use crate::radio_api::*;
use dns_lookup::{lookup_addr, lookup_host};
use once_cell::sync::Lazy;
use radioklw::utils::RadioError;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use seeker::station_data::ServerStats;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::net::IpAddr;

pub static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    static APP_USER_AGENT: &str = concat!("radio_api/", env!("CARGO_PKG_VERSION"),);
    reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap()
});

#[derive(Clone, Debug)]
pub struct Seeker {
    server: String,
}

impl Seeker {
    pub async fn new(dns_lookup: &str) -> Result<Self, RadioError> {
        Ok(Seeker {
            server: Seeker::api_server(dns_lookup).await?,
        })
    }

    pub async fn send<P: DeserializeOwned>(
        self,
        end: &str,
        map: &HashMap<String, String>,
    ) -> Result<P, RadioError> {
        let stations = HTTP_CLIENT
            .post(format!("https://{}{}", &self.server, end))
            .json(map)
            .send()
            .await?
            .json::<P>()
            .await?;

        Ok(stations)
    }

    async fn api_server(dns_lookup_info: &str) -> Result<String, RadioError> {
        let mut ips: Vec<IpAddr> = lookup_host(dns_lookup_info)?;
        ips.shuffle(&mut thread_rng());

        let mut host = String::new();

        for ip in ips {
            host = match lookup_addr(&ip) {
                Ok(hostname) => hostname,
                Err(err) => {
                    eprintln!("Reverse lookup_addr failed for {}: {}", ip, err);
                    continue;
                }
            };

            match Self::check_host_connection(&host).await {
                Ok(_) => {
                    break;
                }
                Err(err) => {
                    eprintln!("Unable to connect to ip: {} (host: {}): {}", ip, host, err);
                }
            }
        }
        Ok(host)
    }

    async fn check_host_connection(dns_lookup_info: &str) -> Result<(), RadioError> {
        let stats: Option<ServerStats> = HTTP_CLIENT
            .get(format!("https://{}{}", dns_lookup_info, "/json/stats"))
            .send()
            .await?
            .json::<Option<ServerStats>>()
            .await?;

        if let Some(stats) = stats {
            if stats.status != "OK" {
                eprintln!("Error: server status: {}", stats.status);
            }
        }

        Ok(())
    }

    pub fn get_radio_station_search_mapper(&self) -> SearchMapper {
        SearchMapper::new(self.clone())
    }
}
