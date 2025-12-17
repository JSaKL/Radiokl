use crate::server_initializer::{start_server, stop_server};
use crate::{chooser, Commands};
use core::time;
use radioklw::utils::{self, RadioError, RadioResult};
use radioklw::{Client, RadioStation, SearchData, Server};
use std::io::ErrorKind;
use std::sync::Arc;
use tokio::io::{self, AsyncWriteExt};
use tokio::net;

const FAVS_FILE: &str = "favs.json";

#[derive(Debug, Clone)]
pub struct Rclient {
    pub socket: std::sync::Arc<tokio::sync::Mutex<net::TcpStream>>,
}

impl Rclient {
    pub async fn new(addr: &str) -> Result<Self, RadioError> {
        if net::TcpStream::connect(addr).await.is_err() {
            println!("Start the local streaming server...");
            start_server().await?;
            tokio::time::sleep(time::Duration::from_secs(2)).await;
        }

        Ok(Rclient {
            socket: std::sync::Arc::new(tokio::sync::Mutex::new(
                net::TcpStream::connect(addr).await?,
            )),
        })
    }

    pub async fn handle_args(&mut self, args: crate::Args) -> RadioResult<()> {
        match args.cmd {
            Commands::Search { name } => {
                let search_data = SearchData {
                    name: name.unwrap_or("".to_owned()),
                    country: args.country.unwrap_or("".to_owned()),
                    language: args.language.unwrap_or("".to_owned()),
                };
                self.send_search_message(&search_data).await?;
                self.recv_message().await?;
            }
            Commands::Stop => {
                self.send_stop_message().await?;
            }
            Commands::Favs => {
                let file_content = get_data_from_file_async().await?;

                if !file_content.is_empty() {
                    let favorites: Vec<RadioStation> = serde_json::from_str(&file_content)?;

                    let mut sviewer =
                        chooser::StationViewer::new(favorites.clone().into(), true, self.clone());
                    if let Some(curr_playing) = sviewer.run_chooser().await? {
                        println!(
                            "<<< Playing Radio station: {} >>>",
                            favorites[curr_playing].name,
                        );
                    }
                } else {
                    println!("Favorites list is empty");
                }
            }
            Commands::StopServer => {
                println!("Stop the local streaming server...");
                self.send_stop_message().await?;
                stop_server().await?;
            }
        }
        Ok(())
    }

    pub async fn send_play_message(&mut self, station: &str) -> RadioResult<()> {
        let req = Some(Client::Play {
            url: Arc::new(station.to_owned()),
        });

        let mut socket = self.socket.lock().await;
        utils::send_json(&mut *socket, &req).await?;
        socket.flush().await?;

        Ok(())
    }

    async fn send_search_message(&mut self, sdata: &SearchData) -> RadioResult<()> {
        let req = Some(Client::Search {
            search_data: Arc::new(sdata.clone()),
        });

        let mut socket = self.socket.lock().await;
        utils::send_json(&mut *socket, &req).await?;
        socket.flush().await?;

        Ok(())
    }

    pub async fn send_stop_message(&mut self) -> RadioResult<()> {
        let req = Some(Client::Stop);

        let mut socket = self.socket.lock().await;
        utils::send_json(&mut *socket, &req).await?;
        socket.flush().await?;

        Ok(())
    }

    async fn recv_message(&self) -> RadioResult<()> {
        let msg = {
            let mut socket = self.socket.lock().await;
            let mut buf = io::BufReader::new(&mut *socket);
            utils::receive_one(&mut buf).await?
        };

        if let Some(msg) = msg {
            match msg {
                Server::RadioChList { radio_list } => {
                    let mut sviewer =
                        chooser::StationViewer::new(radio_list.clone(), false, self.clone());
                    if let Some(curr_playing) = sviewer.run_chooser().await? {
                        println!(
                            "<<< Playing Radio station: {} >>>",
                            radio_list[curr_playing].name,
                        );
                    }
                }
                Server::Error(message) => {
                    return Err(message.into());
                }
            }
        }

        Ok(())
    }

    pub async fn delete_station_from_favorites(&mut self, selected: usize) -> RadioResult<()> {
        let file_content = get_data_from_file_async().await?;

        let mut favorites: Vec<RadioStation> = serde_json::from_str(&file_content)?;

        favorites.remove(selected);

        tokio::fs::write(FAVS_FILE, serde_json::to_string_pretty(&favorites)?).await?;

        Ok(())
    }

    pub async fn save_station_to_file(&mut self, station: &RadioStation) -> RadioResult<()> {
        let file_content = get_data_from_file_async().await?;

        let mut favorites: Vec<RadioStation> = Vec::new();

        if !file_content.is_empty() {
            favorites = serde_json::from_str(&file_content)?;
        }

        favorites.push(station.clone());

        tokio::fs::write(FAVS_FILE, serde_json::to_string_pretty(&favorites)?).await?;

        Ok(())
    }
}

async fn get_data_from_file_async() -> RadioResult<String> {
    // Try to open the file, create if it doesn't exist
    match tokio::fs::File::open(FAVS_FILE).await {
        Ok(_) => {
            // File exists, read its contents
            Ok(tokio::fs::read_to_string(FAVS_FILE).await?)
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                // Create the file if it doesn't exist
                tokio::fs::File::create(FAVS_FILE).await?;
                Ok(String::new())
            }
            _ => {
                let err = format!("failed to open the file {}: {:?}", FAVS_FILE, error);
                Err(err.into())
            }
        },
    }
}
