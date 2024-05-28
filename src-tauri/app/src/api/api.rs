use models::{AccessToken, ChannelId, PostId, Thread};
use reqwest::header::HeaderMap;
use reqwest::{Client, Method};
use serde::Serialize;
use url::Url;

use crate::api::call_event::*;
use crate::errors::Error::ApiError;
use crate::errors::*;

pub async fn handle_request(
    client: &Client,
    server_url: &Url,
    event: &ApiEvent,
    token: Option<&AccessToken>,
) -> Result<Response, Error> {
    let server_url = server_url.join("api/v4/").unwrap();
    match event {
        ApiEvent::LoginEvent(login_id, password) => {
            login(client, server_url, &login_id, &password).await
        }
        ApiEvent::MyTeams => my_teams(client, server_url, token).await,
        ApiEvent::MyTeamMembers => my_team_members(client, server_url, token).await,
        ApiEvent::MyChannels => my_channels(client, server_url, token).await,
        ApiEvent::PostThreads(post_id) => {
            fetch_post_threads(client, server_url, token, post_id).await
        }
    }
}

async fn handle<T: Serialize>(
    client: &Client,
    method: Method,
    url: Url,
    payload: Option<T>,
    token: Option<&AccessToken>,
) -> reqwest::Response {
    let mut builder = client.request(method, url);
    builder = match payload {
        Some(json) => builder.json(&json),
        _ => builder,
    };
    builder = match token {
        Some(bearer_token) => builder.bearer_auth(bearer_token.as_str()),
        _ => builder,
    };
    builder.send().await.unwrap()
}

async fn login(
    client: &Client,
    uri: Url,
    login: &String,
    password: &String,
) -> Result<Response, Error> {
    tracing::info!("Login user: {} to {}", login, uri);
    let login_request = LoginRequest {
        login_id: login.to_string(),
        password: password.to_string(),
    };
    let response = handle(
        client,
        Method::POST,
        uri.join("users/login").unwrap(),
        Some(login_request),
        None,
    )
    .await;
    if !response.status().is_success() {
        tracing::error!("Failed to perform Login body: {:?}", &response.status());
        return match &response.json::<ServerApiError>().await {
            Ok(e) => Err(ApiError(e.to_owned()))?,
            Err(e) => {
                tracing::warn!("Failed to perform login: {e}");
                Err(NativeError::PerformLogin)?
            }
        };
    }
    let token =
        AccessToken::new(get_token(&response.headers()).to_owned()).expect("Invalid access token");
    let user_response = &response.json::<UserResponse>().await;
    tracing::debug!("user response: {user_response:?}");
    match user_response {
        Ok(user) => {
            tracing::info!("Login successful: {user:?}");
            let UserResponse { id, username, .. } = user;
            Ok(Response::LoginResponse(
                token,
                id.to_owned(),
                username.to_owned(),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to perform login: {e}");
            Err(NativeError::PerformLogin)?
        }
    }
}

fn get_token(headers: &HeaderMap) -> &str {
    headers
        .get("token")
        .and_then(|header| header.to_str().ok())
        .unwrap_or_default()
}

async fn my_teams(
    client: &Client,
    uri: Url,
    token: Option<&AccessToken>,
) -> Result<Response, Error> {
    tracing::info!("Get my teams: {}", uri);
    let response = handle(
        client,
        Method::GET,
        uri.join("users/me/teams").unwrap(),
        None as Option<()>,
        token,
    )
    .await;
    if response.status().is_success() {
        let teams: Vec<Team> = response.json::<Vec<Team>>().await.unwrap();
        tracing::trace!("Received my teams: {:?}", teams);
        Ok(Response::MyTeams(teams))
    } else {
        tracing::error!("Failed to get my teams!");
        Err(NativeError::FetchTeams)?
    }
}

async fn my_team_members(
    client: &Client,
    uri: Url,
    token: Option<&AccessToken>,
) -> Result<Response, Error> {
    tracing::info!("Get my team members: {}", uri);
    let response = handle(
        client,
        Method::GET,
        uri.join("users/me/teams/members").unwrap(),
        None as Option<()>,
        token,
    )
    .await;
    if response.status().is_success() {
        let team_members: Vec<TeamMember> = response.json::<Vec<TeamMember>>().await.unwrap();
        tracing::trace!("Received my team members: {:?}", team_members);
        Ok(Response::MyTeamMembers(team_members))
    } else {
        tracing::error!("Failed to get my team members!");
        Err(NativeError::FetchTeamMembers)?
    }
}

async fn my_channels(
    client: &Client,
    uri: Url,
    token: Option<&AccessToken>,
) -> Result<Response, Error> {
    tracing::info!("Get my channels: {}", uri);
    let response = handle(
        client,
        Method::GET,
        uri.join("users/me/channels").unwrap(),
        None as Option<()>,
        token,
    )
    .await;
    if response.status().is_success() {
        let channels: Vec<Channel> = response.json::<Vec<Channel>>().await.unwrap();
        tracing::trace!("Received my channels: {:?}", channels);
        Ok(Response::MyChannels(channels))
    } else {
        tracing::error!("Failed to get my channels!");
        Err(NativeError::FetchChannels)?
    }
}

async fn fetch_channel_posts(
    client: &Client,
    uri: Url,
    token: Option<&AccessToken>,
    channel_id: &ChannelId,
) -> Result<ChannelPayload, Error> {
    let response = handle(
        client,
        Method::GET,
        uri.join(&format!("api/v4/channels/{channel_id}/posts"))
            .unwrap(),
        None as Option<()>,
        token,
    )
    .await;
    if response.status().is_success() {
        let threads: Post = response.json().await.unwrap();
        tracing::trace!("Received threads: {:?}", threads);
        Ok(Response::ChannelThreads(threads))
    } else {
        tracing::error!("Failed to get my channels!");
        Err(NativeError::FetchChannels)?
    }
}

async fn fetch_post_threads(
    client: &Client,
    uri: Url,
    token: Option<&AccessToken>,
    post_id: &PostId,
) -> Result<PostPayload, Error> {
    let response = handle(
        client,
        Method::GET,
        uri.join(&format!("api/v4/posts/{post_id}/thread")).unwrap(),
        None as Option<()>,
        token,
    )
    .await;
    if response.status().is_success() {
        let threads: Vec<Thread> = response.json().await.unwrap();
        tracing::trace!("Received threads: {:?}", threads);
        Ok(Response::ChannelThreads(threads))
    } else {
        tracing::error!("Failed to get my channels!");
        Err(NativeError::FetchChannels)?
    }
}
