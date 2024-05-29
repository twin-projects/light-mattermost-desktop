use models::*;
use reqwest::Client;
use tauri::State;
use tokio::sync::Mutex;
use url::Url;

use crate::api::call_event::*;
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
    tracing::info!("User login");
    let url = {
        server_state_mutex
            .lock()
            .await
            .current
            .as_ref()
            .unwrap()
            .url
            .clone()
    };
    tracing::debug!("current url: {url:?}");
    let result = handle_request(
        &http_client,
        &url,
        &ApiEvent::LoginEvent(login, password),
        None,
    )
    .await;
    tracing::info!("result: {:?}", result);
    let Response::LoginResponse {
        token,
        user_id: _id,
        user_name,
    } = result?
    else {
        return Err(NativeError::UnexpectedResponse)?;
    };
    tracing::info!("Authorized");
    {
        user_state_mutex.lock().await.token = Some(token.to_owned());
    }
    Ok(UserDetails {
        username: user_name.to_owned(),
    })
}

#[tauri::command]
pub async fn my_teams(
    user_state_mutex: State<'_, Mutex<UserState>>,
    server_state_mutex: State<'_, Mutex<ServerState>>,
    http_client: State<'_, Client>,
) -> Result<Vec<Team>, Error> {
    let token_option = { user_state_mutex.lock().await.token.as_ref().cloned() };
    let server_state = server_state_mutex.lock().await;
    let current_url = server_state.current.as_ref().unwrap();
    let result = handle_request(
        &http_client,
        &current_url.url,
        &ApiEvent::MyTeams,
        token_option.as_ref(),
    )
    .await?;
    let Response::MyTeams(teams) = result else {
        return Err(NativeError::UnexpectedResponse)?;
    };
    let mut user_state = user_state_mutex.lock().await;
    user_state.teams = Some(teams.to_owned());
    Ok(teams.to_owned())
}

#[tauri::command]
pub async fn my_team_members(
    user_state_mutex: State<'_, Mutex<UserState>>,
    server_state_mutex: State<'_, Mutex<ServerState>>,
    http_client: State<'_, Client>,
) -> Result<Vec<TeamMember>, Error> {
    let token_option = { user_state_mutex.lock().await.token.as_ref().cloned() };
    let server_state = server_state_mutex.lock().await;
    let current_url = server_state.current.as_ref().unwrap();
    let result = handle_request(
        &http_client,
        &current_url.url,
        &ApiEvent::MyTeamMembers,
        token_option.as_ref(),
    )
    .await?;
    let Response::MyTeamMembers(team_members) = result else {
        return Err(NativeError::UnexpectedResponse)?;
    };
    let mut user_state = user_state_mutex.lock().await;
    user_state.team_members = Some(team_members.to_owned());
    Ok(team_members.to_owned())
}

#[tauri::command]
pub async fn my_channels(
    user_state_mutex: State<'_, Mutex<UserState>>,
    server_state_mutex: State<'_, Mutex<ServerState>>,
    http_client: State<'_, Client>,
) -> Result<Vec<Channel>, Error> {
    let token_option = { user_state_mutex.lock().await.token.as_ref().cloned() };
    let server_state = server_state_mutex.lock().await;
    let current_url = server_state.current.as_ref().unwrap();
    let result = handle_request(
        &http_client,
        &current_url.url,
        &ApiEvent::MyChannels,
        token_option.as_ref(),
    )
    .await?;
    let Response::MyChannels(channels) = result else {
        return Err(NativeError::UnexpectedResponse)?;
    };
    let mut user_state = user_state_mutex.lock().await;
    user_state.channels = Some(channels.to_owned());
    Ok(channels.to_owned())
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
    let mut state = state_mutex.lock().await;
    state.current = Some(current.clone());
    state.servers.push(current.clone());
    tracing::info!("{:?}", state.current);
    tracing::info!("{:?}", state.servers);
    Ok(state.servers.clone())
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct ChangeServerOutput {
    pub current: Server,
    pub list: Vec<Server>,
}

#[tauri::command]
pub async fn change_server(
    server_name: &str,
    state_mutex: State<'_, Mutex<ServerState>>,
) -> Result<ChangeServerOutput, Error> {
    let mut state = state_mutex.lock().await;
    let Some(current) = state
        .servers
        .iter()
        .find(|server| server.name == server_name)
        .cloned()
    else {
        return Err(NativeError::UnknownServer)?;
    };
    state.current = Some(current.clone());
    tracing::info!("{:?}", current);
    tracing::info!("{:?}", state.servers);
    Ok(ChangeServerOutput {
        list: state.servers.clone(),
        current,
    })
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

#[tauri::command]
pub async fn post_threads(
    post_id: PostId,
    user_state_mutex: State<'_, Mutex<UserState>>,
    server_state_mutex: State<'_, Mutex<ServerState>>,
    http_client: State<'_, Client>,
) -> Result<PostThread, Error> {
    let state = server_state_mutex.lock().await;
    let token = user_state_mutex.lock().await.token.clone();
    let client = &*http_client;
    let server_url = state
        .current
        .as_ref()
        .ok_or_else(|| NativeError::ServerNotSelected)?
        .url
        .to_owned();
    let v = handle_request(
        client,
        &server_url,
        &ApiEvent::PostThreads(post_id),
        token.as_ref(),
    )
    .await?;
    let Response::ChannelThreads(v) = v else {
        return Err(Error::Native(NativeError::UnexpectedResponse));
    };
    Ok(v)
}

#[tauri::command]
pub async fn channel_posts(
    channel_id: ChannelId,
    user_state_mutex: State<'_, Mutex<UserState>>,
    server_state_mutex: State<'_, Mutex<ServerState>>,
    http_client: State<'_, Client>,
) -> Result<PostThread, Error> {
    let state = server_state_mutex.lock().await;
    let token = user_state_mutex.lock().await.token.clone();
    let client = &*http_client;
    let server_url = state
        .current
        .as_ref()
        .ok_or_else(|| NativeError::ServerNotSelected)?
        .url
        .to_owned();
    let v = handle_request(
        client,
        &server_url,
        &ApiEvent::ChannelPosts(channel_id),
        token.as_ref(),
    )
    .await?;
    let Response::ChannelPosts(v) = v else {
        return Err(Error::Native(NativeError::UnexpectedResponse));
    };
    Ok(v)
}
