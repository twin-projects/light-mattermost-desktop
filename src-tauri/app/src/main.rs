// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::Client;
use tokio::sync::Mutex;

use crate::commands::*;
use crate::errors::*;
use crate::states::{ServerState, UserState};

mod api;
mod commands;
pub mod errors;
mod states;
pub mod storage;

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        Error::PoisonError(err.to_string())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tauri::Builder::default()
        .manage(Client::new())
        .manage(Mutex::new(UserState::default()))
        .manage(Mutex::new(ServerState::default()))
        .manage(storage::Storage::new())
        .on_page_load(|window, _load_payload| {
            window.open_devtools();
            // window.close_devtools();
            //
        })
        .invoke_handler(tauri::generate_handler![
            login,
            logout,
            add_server,
            get_current_server,
            get_all_servers,
            my_teams,
            my_team_members,
            my_channels,
            change_server,
            post_threads,
            channel_posts,
            user_unseen,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
