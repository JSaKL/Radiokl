use radioklw::utils::RadioResult;
use std::io::{stdin, stdout, Write};
use std::sync::Arc;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};

use radioklw::RadioStation;

use crate::rclient::Rclient;

#[derive(Debug)]
struct Coordinates {
    pub x: usize,
    pub y: usize,
}

pub struct StationViewer {
    radio_list: Arc<Vec<RadioStation>>,
    radio_list_len: usize,
    cur_pos: Coordinates,
    terminal_size: Coordinates,
    favorites_menu: bool,
    curr_playing: Option<usize>,
    rclient: Rclient,
}

impl StationViewer {
    pub fn new(radios: Arc<Vec<RadioStation>>, show_favorites: bool, rclient: Rclient) -> Self {
        let radio_list_len = radios.len();
        let size = termion::terminal_size().unwrap_or_default();
        Self {
            radio_list: radios,
            radio_list_len,
            cur_pos: Coordinates {
                x: 1,
                y: radio_list_len,
            },
            terminal_size: Coordinates {
                x: size.0 as usize,
                y: size.1 as usize,
            },
            favorites_menu: show_favorites,
            curr_playing: None,
            rclient,
        }
    }

    async fn show_list(&mut self, is_action: bool) {
        let ill_char = '\n';
        let mut footer_str =
            ">>> Options: Ctrl-p to Play, Ctrl-s to Stop, Ctrl-w to Save, Ctrl-q to Quit";

        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        let playing_now = if let Some(cpl) = self.curr_playing {
            self.radio_list[cpl].name.replace(ill_char, "")
        } else {
            "-".to_string()
        };

        if self.favorites_menu {
            footer_str =
                ">>> Options: Ctrl-p to Play, Ctrl-s to Stop, Ctrl-d to Delete, Ctrl-q to Quit";
            if is_action {
                println!(
                    "{}{}*** {} <<<< DELETED >>>>\r{}",
                    color::Bg(color::Black),
                    color::Fg(color::LightCyan),
                    self.radio_list[self.cur_pos.y - 1].name,
                    style::Reset
                );
            } else {
                println!(
                    "{}{}({} stations found) Playing now: {}\r{}",
                    color::Bg(color::Black),
                    color::Fg(color::LightGreen),
                    self.radio_list_len,
                    playing_now,
                    style::Reset
                );
            }
        } else if is_action {
            println!(
                "{}{}*** {} <<<< SAVED >>>>\r{}",
                color::Bg(color::Black),
                color::Fg(color::LightCyan),
                self.radio_list[self.cur_pos.y - 1].name,
                style::Reset
            );
        } else {
            println!(
                "{}{}({} stations found) Playing now: {}\r{}",
                color::Bg(color::Black),
                color::Fg(color::LightGreen),
                self.radio_list_len,
                playing_now,
                style::Reset
            );
        }

        let mut show_cursor = false;

        if self.radio_list_len < self.terminal_size.y {
            for line in 0..self.radio_list_len {
                println!(
                    "{}: {}, Country: {}, Language: {}\r",
                    line + 1,
                    self.radio_list[line].name,
                    self.radio_list[line].country,
                    self.radio_list[line].language,
                );
            }
            show_cursor = true
        } else if self.cur_pos.y < self.terminal_size.y
            && self.cur_pos.y <= (self.terminal_size.y - 3)
        {
            for line in 0..self.terminal_size.y - 3 {
                println!(
                    "{}: {}, Country: {}, Language: {}\r",
                    line + 1,
                    self.radio_list[line].name,
                    self.radio_list[line].country,
                    self.radio_list[line].language
                );
            }
            show_cursor = true
        } else {
            for line in self.cur_pos.y - (self.terminal_size.y - 3)..self.cur_pos.y {
                println!(
                    "{}: {}, Country: {}, Language: {}\r",
                    line + 1,
                    self.radio_list[line].name,
                    self.radio_list[line].country,
                    self.radio_list[line].language
                );
            }
        }

        println!(
            "{}",
            termion::cursor::Goto(0, (self.terminal_size.y - 2) as u16),
        );

        println!(
            "{}{}{}{}",
            color::Bg(color::Black),
            color::Fg(color::LightGreen),
            footer_str,
            style::Reset
        );

        if !show_cursor {
            println!(
                "{}",
                termion::cursor::Goto(0, (self.terminal_size.y - 3) as u16),
            );
        } else {
            println!(
                "{}",
                termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
            );
        }
    }

    fn inc_y(&mut self) {
        if self.cur_pos.y < self.radio_list_len {
            self.cur_pos.y += 1;
        }
        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16),
        );
    }

    fn dec_y(&mut self) {
        if self.cur_pos.y > 1 {
            self.cur_pos.y -= 1;
        }
        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16),
        );
    }

    fn set_pos(&mut self, _x: usize, y: usize) {
        self.cur_pos.y = y;
        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
        );
    }

    async fn run(&mut self) -> RadioResult<()> {
        let mut stdout = stdout().into_raw_mode()?;
        let stdin = stdin();
        for key in stdin.keys() {
            match key.unwrap_or(Key::Ctrl('q')) {
                Key::Ctrl('q') => {
                    break;
                }
                Key::Ctrl('p') => {
                    self.curr_playing = Some(self.cur_pos.y - 1);
                    self.rclient
                        .send_play_message(&self.radio_list[self.cur_pos.y - 1].url)
                        .await?;
                    self.show_list(false).await;
                }
                Key::Ctrl('s') => {
                    self.curr_playing = None;
                    self.rclient.send_stop_message().await?;
                    self.show_list(false).await;
                }
                Key::Ctrl('w') => {
                    if !self.favorites_menu {
                        self.rclient
                            .save_station_to_file(&self.radio_list[self.cur_pos.y - 1])
                            .await?;
                        self.show_list(true).await;
                    }
                }
                Key::Ctrl('d') => {
                    if self.favorites_menu {
                        self.rclient
                            .delete_station_from_favorites(self.cur_pos.y - 1)
                            .await?;
                        self.show_list(true).await;
                    }
                }
                Key::Up => {
                    self.dec_y();
                    self.show_list(false).await;
                }
                Key::Down => {
                    self.inc_y();
                    self.show_list(false).await;
                }
                _ => {}
            }
            stdout.flush()?;
        }
        Ok(())
    }

    pub async fn run_chooser(&mut self) -> RadioResult<Option<usize>> {
        println!("{}", termion::clear::All);
        println!("{}", termion::cursor::Show);
        println!("{}", termion::cursor::Goto(1, 1));
        self.set_pos(1, 1);
        self.show_list(false).await;
        self.run().await?;
        print!(
            "{}",
            termion::cursor::Goto(1, (self.terminal_size.y + 3).try_into()?)
        );
        if self.curr_playing.is_some() {
            return Ok(self.curr_playing);
        }

        Ok(None)
    }
}
