use reqwest::Client;
use tauri::State;
use tokio::sync::Mutex;
use url::Url;

use crate::api::call_event::{ApiEvent, Response, Team, UserDetails};
use crate::api::handle_request;
use crate::errors::{Error, NativeError};
use crate::states::{Server, ServerState, UserState};

#[tauri::command]
pub async fn login(
    login: String,
    password: String,
    user_state_mutex: State<'_, Mutex<UserState>>,
    server_state_mutex: State<'_, Mutex<ServerState>>,
    http_client: State<'_, Client>,
) -> Result<UserDetails, Error> {
    tracing::info!("{}", "User login ".to_string());
    let mut user_state = user_state_mutex.lock().await;
    let server_state = server_state_mutex.lock().await;
    let current_url = server_state.current.as_ref().unwrap();
    let result = handle_request(
        &http_client,
        &current_url.url,
        &ApiEvent::LoginEvent(login, password),
        None,
    )
    .await?;
    let Response::LoginResponse(token, _id, username) = result else {
        return Err(NativeError::UnexpectedResponse)?;
    };
    tracing::info!("Authorized");
    user_state.token = Some(token.to_owned());
    Ok(UserDetails {
        username: username.to_owned(),
    })
}

#[tauri::command]
pub async fn my_teams(
    user_state_mutex: State<'_, Mutex<UserState>>,
    server_state_mutex: State<'_, Mutex<ServerState>>,
    http_client: State<'_, Client>,
) -> Result<Vec<Team>, Error> {
    let mut user_state = user_state_mutex.lock().await;
    let token_option = user_state.token.as_ref();
    let server_state = server_state_mutex.lock().await;
    let current_url = server_state.current.as_ref().unwrap();
    let result = handle_request(
        &http_client,
        &current_url.url,
        &ApiEvent::MyTeams,
        token_option,
    )
    .await?;
    let Response::MyTeams(teams) = result else {
        return Err(NativeError::UnexpectedResponse)?;
    };
    user_state.teams = Some(teams.to_owned());
    Ok(teams.to_owned())
}

#[tauri::command]
pub async fn logout(state_mutex: State<'_, Mutex<UserState>>) -> Result<(), Error> {
    let mut server_state = state_mutex.lock().await;
    server_state.user_details = None;
    server_state.token = None;
    Ok(())
}

#[tauri::command]
pub async fn add_server(
    name: &str,
    url: &str,
    state_mutex: State<'_, Mutex<ServerState>>,
) -> Result<Vec<Server>, ()> {
    let mut state = state_mutex.lock().await;
    let current = match Url::parse(url) {
        Ok(url) => Server {
            name: name.to_owned(),
            url,
        },
        Err(e) => {
            tracing::warn!("Invalid url {url:?}: {e}");
            return Err(());
        }
    };
    state.current = Some(current.clone());
    state.servers.push(current.clone());
    Ok(state.servers.clone())
}

#[tauri::command]
pub async fn get_current_server(
    state_mutex: State<'_, Mutex<ServerState>>,
) -> Result<Server, Error> {
    let state = state_mutex.lock().await;
    let current = state
        .current
        .as_ref()
        .ok_or_else(|| NativeError::ServerNotSelected)?
        .to_owned();
    tracing::debug!("Current selected server {:?}", current);
    Ok(current)
}

#[tauri::command]
pub async fn get_all_servers(
    state_mutex: State<'_, Mutex<ServerState>>,
) -> Result<Vec<Server>, Error> {
    let state = state_mutex.lock().await;
    let servers = state.servers.to_owned();
    tracing::debug!("all servers: {:?}", servers);
    Ok(servers)
}

