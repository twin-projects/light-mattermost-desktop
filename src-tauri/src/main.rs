// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::error;
use reqwest::Client;
use serde::Serialize;
use tauri::State;
use tokio::sync::Mutex;

use api::call_event::{ApiEvent, Response};
use api::handle_request;

use crate::api::call_event::{UserDetails, UserResponse};

mod api;

#[derive(Debug, thiserror::Error)]
enum NativeError {
    #[error("No mattermost server is selected")]
    ServerNotSelected,
    #[error("Unexpected response from mattermost server")]
    UnexpectedResponse,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Native(#[from] NativeError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Standard(#[from] core::fmt::Error),
    #[error("the mutex was poisoned")]
    PoisonError(String),
}

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

#[derive(Serialize, Clone)]
pub(crate) struct UserState {
    #[serde(skip_serializing)]
    token: Option<String>,
    user_details: Option<UserDetails>,
}

impl Default for UserState {
    fn default() -> Self {
        Self {
            token: None,
            user_details: None,
        }
    }
}

#[derive(Serialize, Clone)]
pub(crate) struct ServerState {
    #[serde(skip_serializing)]
    current: Option<String>,
    urls: Vec<String>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            current: Some("http://localhost:8065".to_owned()), // TODO add dev env
            urls: vec![],
        }
    }
}

#[tauri::command]
async fn login(
    login: String,
    password: String,
    state_mutex: State<'_, Mutex<UserState>>,
    server_state_mutex: State<'_, Mutex<ServerState>>,
    http_client: State<'_, Client>,
) -> Result<UserDetails, Error> {
    let server_state = server_state_mutex.lock().await;
    let current_url = server_state
        .current
        .as_ref()
        .ok_or_else(|| NativeError::ServerNotSelected)?;
    let result = handle_request(
        &http_client,
        current_url,
        &ApiEvent::LoginEvent(login, password),
    )
    .await?;
    let Response::LoginResponse(token, user) = result else {
        return Err(NativeError::UnexpectedResponse)?;
    };
    let mut state = state_mutex.lock().await;
    state.token = Some(token);
    let UserResponse {
        username,
        email,
        nickname,
        first_name,
        last_name,
        roles,
        ..
    } = user;
    let user_details = UserDetails {
        username,
        email,
        nickname,
        first_name,
        last_name,
        roles,
    };
    state.user_details = Some(user_details.clone());
    Ok(user_details)
}

#[tauri::command]
async fn logout(state_mutex: State<'_, Mutex<UserState>>) -> Result<(), Error> {
    let mut server_state = state_mutex.lock().await;
    server_state.user_details = None;
    server_state.token = None;
    Ok(())
}

#[tauri::command]
async fn add_server(url: &str, state_mutex: State<'_, Mutex<ServerState>>) -> Result<String, ()> {
    let mut state = state_mutex.lock().await;
    let current: String = url.to_owned();
    state.current = Some(current.to_owned());
    state.urls.push(current.to_owned());
    Ok(current)
}

#[tauri::command]
async fn get_current_server(state_mutex: State<'_, Mutex<ServerState>>) -> Result<String, ()> {
    let state = state_mutex.lock().await;
    let current = state.current.as_ref().unwrap().to_owned();
    Ok(current)
}

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
async fn main() {
    let client: Client = Client::new();
    tauri::Builder::default()
        .manage(client)
        .manage(Mutex::new(UserState::default()))
        .manage(Mutex::new(ServerState::default()))
        .invoke_handler(tauri::generate_handler![
            login,
            logout,
            add_server,
            get_current_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
