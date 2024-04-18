use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync::{Arc, Mutex};
use radioklw::utils::{self, RadioError, RadioResult};
use radioklw::{Client, Server};

use crate::player;
use crate::radio_api::Seeker;
use crate::radio_api::StationOrder;
use player::*;

const INDEX_URL: &str = "all.api.radio-browser.info";

#[derive(Debug, Clone)]
pub struct Connection {
    pub is_playing: bool,
    pub player: Player,
    pub seeker: Seeker,
}

impl Connection {
    pub async fn new() -> Result<Self, RadioError> {
        Ok(Connection {
            is_playing: false,
            player: Player::new(),
            seeker: Seeker::new(INDEX_URL).await?,
        })
    }

    pub async fn handle(&mut self, socket: TcpStream) -> RadioResult<()> {
        let sender = Arc::new(Sender::new(socket.clone()));
        let buffered = BufReader::new(socket);
        let mut from_client = utils::receive(buffered);

        while let Some(req_res) = from_client.next().await {
            let request = req_res?;

            let result = match request {
                Client::Play { url } => {
                    if self.is_playing {
                        self.player.stop().await?;
                        self.is_playing = false;
                    }

                    self.player.play(url).await?;
                    self.is_playing = true;
                    Ok(())
                }
                Client::Stop => {
                    if self.is_playing {
                        self.player.stop().await?;
                        self.is_playing = false;
                    }

                    Ok(())
                }
                Client::Search { search_data } => {
                    let stations = self
                        .seeker
                        .get_radio_station_search_mapper()
                        .name(search_data.name.clone())
                        .country(search_data.country.clone())
                        .language(search_data.language.clone())
                        .reverse(true)
                        .order(StationOrder::Clickcount)
                        .send();
                    let station_md = stations.await?;

                    if !station_md.is_empty() {
                        let rlist = Server::RadioChList {
                            radio_list: Arc::new(station_md),
                        };
                        sender.send(rlist).await?;
                        Ok(())
                    } else {
                        Err("No radio stations found.".to_string())
                    }
                }
            };
            if let Err(message) = result {
                //eprintln!("Error: message: {}", message);
                let report = Server::Error(message);
                sender.send(report).await?;
            }
        }
        Ok(())
    }
}

pub struct Sender(Mutex<TcpStream>);

impl Sender {
    pub fn new(client: TcpStream) -> Sender {
        Sender(Mutex::new(client))
    }

    pub async fn send(&self, packet: Server) -> RadioResult<()> {
        let mut tcpstream = self.0.lock().await;
        utils::send_json(&mut *tcpstream, &packet).await?;
        tcpstream.flush().await?;
        Ok(())
    }
}
