// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::Client;
use serde::Serialize;
use tauri::State;
use tokio::sync::Mutex;
use url::Url;

use api::call_event::{ApiEvent, Response};
use api::handle_request;

use crate::api::call_event::{Team, UserDetails};

mod api;

#[derive(Debug, thiserror::Error)]
enum NativeError {
    #[error("No mattermost server is selected")]
    ServerNotSelected,
    #[error("Unexpected response from mattermost server")]
    UnexpectedResponse,
    #[error("Unable to fetch teams from mattermost server")]
    FetchTeams,
    #[error("Unable to perform login, mattermost server return an error")]
    PerformLogin,
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
    #[error("{_0}")]
    Url(#[from] url::ParseError),
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
    teams: Option<Vec<Team>>,
}

impl Default for UserState {
    fn default() -> Self {
        Self {
            token: None,
            user_details: None,
            teams: None,
        }
    }
}

#[derive(Serialize, Clone)]
pub(crate) struct ServerState {
    #[serde(skip_serializing)]
    current: Option<Url>,
    urls: Vec<Url>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            current: Url::parse("http://localhost:8065").ok(), // TODO add dev env
            urls: vec![],
        }
    }
}

struct FeValue<T> {
    payload: T,
}

#[tauri::command]
async fn login(
    login: String,
    password: String,
    user_state_mutex: State<'_, Mutex<UserState>>,
    server_state_mutex: State<'_, Mutex<ServerState>>,
    http_client: State<'_, Client>,
) -> Result<UserDetails, Error> {
    let mut user_state = user_state_mutex.lock().await;
    let server_state = server_state_mutex.lock().await;
    let current_url = server_state.current.as_ref().unwrap();
    let result = handle_request(
        &http_client,
        current_url,
        &ApiEvent::LoginEvent(login, password),
        None,
    )
    .await?;
    match &response {
        Response::LoginResponse(token, _id, username) => {
            user_state.token = Some(token.to_owned());
            Ok(UserDetails {
                username: username.to_owned(),
            })
        }
        Response::MyTeams(_) => Err(Error::PoisonError("Incorrect type".to_string())),
    }
}

#[tauri::command]
async fn my_teams(
    user_state_mutex: State<'_, Mutex<UserState>>,
    server_state_mutex: State<'_, Mutex<ServerState>>,
    http_client: State<'_, Client>,
) -> Result<Vec<Team>, Error> {
    let mut user_state = user_state_mutex.lock().await;
    let token_option = user_state.token.as_ref();
    let server_state = server_state_mutex.lock().await;
    let current_url = server_state.current.as_ref().unwrap();
    let result =
        handle_request(&http_client, current_url, &ApiEvent::MyTeams, token_option).await?;
    match &response {
        Response::LoginResponse(token, id, username) => {
            Err(Error::PoisonError("Incorrect type".to_string()))
        }
        Response::MyTeams(teams) => {
            user_state.teams = Some(teams.to_owned());
            Ok(teams.to_owned())
        }
    }
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
    let current = match Url::parse(url) {
        Ok(url) => url,
        Err(e) => {
            tracing::warn!("Invalid url {url:?}: {e}");
            return Err(());
        }
    };
    state.current = Some(current.clone());
    state.urls.push(current.clone());
    Ok(current.into())
}

#[tauri::command]
async fn get_current_server(state_mutex: State<'_, Mutex<ServerState>>) -> Result<String, ()> {
    let state = state_mutex.lock().await;
    let current = state.current.as_ref().unwrap().to_owned();
    Ok(current.into())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let client: Client = Client::new();
    tauri::Builder::default()
        .manage(client)
        .manage(Mutex::new(UserState::default()))
        .manage(Mutex::new(ServerState::default()))
        .invoke_handler(tauri::generate_handler![
            login,
            logout,
            add_server,
            get_current_server,
            my_teams
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
