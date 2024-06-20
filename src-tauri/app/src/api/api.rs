use models::*;
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
        ApiEvent::Login(login_id, password) => {
            login(client, server_url, &login_id, &password).await
        }
        ApiEvent::MyTeams => my_teams(client, server_url, token).await,
        ApiEvent::MyTeamMembers => my_team_members(client, server_url, token).await,
        ApiEvent::MyChannels => my_channels(client, server_url, token).await,
        ApiEvent::PostThreads(post_id) => {
            fetch_post_thread(client, server_url, token, post_id).await
        }
        ApiEvent::ChannelPosts(channel_id) => {
            fetch_channel_posts(client, server_url, token, channel_id).await
        }
        ApiEvent::UserUnread {
            channel_id,
            user_id,
        } => fetch_user_unread(client, server_url, token, user_id, channel_id).await,
        ApiEvent::Users { page, per_page } => {
            users(client, server_url, token, *page, *per_page).await
        }
    }
}

async fn handle<T: Serialize>(
    client: &Client,
    method: Method,
    url: Url,
    payload: Option<T>,
    token: Option<&AccessToken>,
) -> Result<reqwest::Response, reqwest::Error> {
    let mut builder = client.request(method, url);
    builder = match payload {
        Some(json) => builder.json(&json),
        _ => builder,
    };
    builder = match token {
        Some(bearer_token) => builder.bearer_auth(bearer_token.as_str()),
        _ => builder,
    };
    builder.send().await
}

async fn login(
    client: &Client,
    uri: Url,
    login: &String,
    password: &String,
) -> Result<Response, Error> {
    tracing::info!("Login user: {} to {}", login, uri);
    let login_request = LoginRequest {
        login_id: Login::new(login.to_string()).expect("Invalid login"),
        password: Pass::new(password.to_string()).expect("Invalid password"),
    };
    let result = handle(
        client,
        Method::POST,
        uri.join("users/login").unwrap(),
        Some(login_request),
        None,
    )
    .await
    .map_err(|error| {
        Err(Error::RequestFailed(ClientFailed {
            reason: error.to_string(),
        }))
    });
    match result {
        Ok(response) => {
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
            let token = AccessToken::new(get_token(&response.headers()).to_owned())
                .expect("Invalid access token");
            let user_response = &response.json::<UserResponse>().await;
            tracing::debug!("user response: {user_response:?}");
            match user_response {
                Ok(user) => {
                    tracing::info!("Login successful: {user:?}");
                    let UserResponse { id, username, .. } = user;
                    Ok(Response::Login {
                        token,
                        user_id: id.to_owned(),
                        user_name: username.to_owned(),
                    })
                }
                Err(e) => {
                    tracing::error!("Failed to perform login: {e}");
                    Err(NativeError::PerformLogin)?
                }
            }
        }
        Err(error) => error,
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
    let result = handle(
        client,
        Method::GET,
        uri.join("users/me/teams").unwrap(),
        None as Option<()>,
        token,
    )
    .await
    .map_err(|error| {
        Err(Error::RequestFailed(ClientFailed {
            reason: error.to_string(),
        }))
    });
    match result {
        Ok(response) => {
            if response.status().is_success() {
                let teams: Vec<Team> = response.json::<Vec<Team>>().await.unwrap();
                tracing::trace!("Received my teams: {:?}", teams);
                Ok(Response::MyTeams(teams))
            } else {
                tracing::error!("Failed to get my teams!");
                Err(NativeError::FetchTeams)?
            }
        }
        Err(error) => error,
    }
}

async fn my_team_members(
    client: &Client,
    uri: Url,
    token: Option<&AccessToken>,
) -> Result<Response, Error> {
    tracing::info!("Get my team members: {}", uri);
    let result = handle(
        client,
        Method::GET,
        uri.join("users/me/teams/members").unwrap(),
        None as Option<()>,
        token,
    )
    .await
    .map_err(|error| {
        Err(Error::RequestFailed(ClientFailed {
            reason: error.to_string(),
        }))
    });
    match result {
        Ok(response) => {
            if response.status().is_success() {
                let team_members: Vec<TeamMember> =
                    response.json::<Vec<TeamMember>>().await.unwrap();
                tracing::trace!("Received my team members: {:?}", team_members);
                Ok(Response::MyTeamMembers(team_members))
            } else {
                tracing::error!("Failed to get my team members!");
                Err(NativeError::FetchTeamMembers)?
            }
        }
        Err(error) => error,
    }
}

async fn my_channels(
    client: &Client,
    uri: Url,
    token: Option<&AccessToken>,
) -> Result<Response, Error> {
    tracing::info!("Get my channels: {}", uri);
    let result = handle(
        client,
        Method::GET,
        uri.join("users/me/channels").unwrap(),
        None as Option<()>,
        token,
    )
    .await
    .map_err(|error| {
        Err(Error::RequestFailed(ClientFailed {
            reason: error.to_string(),
        }))
    });
    match result {
        Ok(response) => {
            if response.status().is_success() {
                let channels = response.json::<Vec<Channel>>().await.unwrap();
                tracing::trace!("Received my channels: {:?}", channels);
                Ok(Response::MyChannels(channels))
            } else {
                tracing::error!("Failed to get my channels!");
                Err(NativeError::FetchChannels)?
            }
        }
        Err(error) => error,
    }
}

async fn fetch_channel_posts(
    client: &Client,
    uri: Url,
    token: Option<&AccessToken>,
    channel_id: &ChannelId,
) -> Result<Response, Error> {
    let result = handle(
        client,
        Method::GET,
        uri.join(&format!("channels/{channel_id}/posts")).unwrap(),
        None as Option<()>,
        token,
    )
    .await
    .map_err(|error| {
        Error::RequestFailed(ClientFailed {
            reason: error.to_string(),
        })
    })?;
    let response = result.text().await;
    let txt = response.map_err(|err| {
        tracing::error!("Channel posts returned: {err}");
        NativeError::FetchChannels
    })?;
    let posts = serde_json::from_str::<PostThread>(&txt).map_err(|err| {
        tracing::info!("Channel posts txt: {txt}");
        tracing::error!("Channel posts parse: {err}");
        NativeError::FetchChannels
    })?;
    Ok(Response::ChannelPosts(posts))
}

async fn fetch_post_thread(
    client: &Client,
    uri: Url,
    token: Option<&AccessToken>,
    post_id: &PostId,
) -> Result<Response, Error> {
    let result = handle(
        client,
        Method::GET,
        uri.join(&format!("posts/{post_id}/thread")).unwrap(),
        None as Option<()>,
        token,
    )
    .await
    .map_err(|error| {
        Err(Error::RequestFailed(ClientFailed {
            reason: error.to_string(),
        }))
    });
    match result {
        Ok(response) => {
            if response.status().is_success() {
                let threads: PostThread = response.json().await.unwrap();
                tracing::trace!("Received threads: {:?}", threads);
                Ok(Response::ChannelThreads(threads))
            } else {
                tracing::error!("Failed to get my channels!");
                Err(NativeError::FetchPosts)?
            }
        }
        Err(error) => error,
    }
}

pub async fn fetch_user_unread(
    client: &Client,
    uri: Url,
    token: Option<&AccessToken>,
    user_id: &UserId,
    channel_id: &ChannelId,
) -> Result<Response, Error> {
    let url = uri.join(&format!("users/{user_id}/channels/{channel_id}/posts/unread?limit_after=30&limit_before=30&skipFetchThreads=false&collapsedThreads=true&collapsedThreadsExtended=false")).unwrap();
    tracing::info!("Requesting: {url:?}");
    let result = handle(client, Method::GET, url, None as Option<()>, token)
        .await
        .map_err(|error| {
            Error::RequestFailed(ClientFailed {
                reason: error.to_string(),
            })
        })?;
    let response = result.text().await;
    let txt = response.map_err(|err| {
        tracing::error!("Channel posts returned: {err}");
        NativeError::FetchChannels
    })?;
    let posts = serde_json::from_str::<PostThread>(&txt).map_err(|err| {
        tracing::info!("Channel posts txt: {txt}");
        tracing::error!("Channel posts parse: {err}");
        NativeError::FetchChannels
    })?;
    Ok(Response::ChannelPosts(posts))
}

pub async fn users(
    client: &Client,
    uri: Url,
    token: Option<&AccessToken>,
    page: Option<u32>,
    per_page: Option<u32>,
) -> Result<Response, Error> {
    let url = uri
        .join(&format!(
            "users?page={page}&per_page={per_page}",
            page = page.unwrap_or_default(),
            per_page = per_page.unwrap_or(100)
        ))
        .unwrap();
    tracing::info!("Requesting: {url:?}");
    let result = handle(client, Method::GET, url, None as Option<()>, token)
        .await
        .map_err(|error| {
            Error::RequestFailed(ClientFailed {
                reason: error.to_string(),
            })
        })?;
    let response = result.text().await;
    let txt = response.map_err(|err| {
        tracing::error!("Users returned: {err}");
        NativeError::FetchUsers
    })?;
    let users = serde_json::from_str::<Vec<UserResponse>>(&txt).map_err(|err| {
        tracing::info!("Users txt: {txt}");
        tracing::error!("Users parse: {err}");
        NativeError::FetchUsers
    })?;
    Ok(Response::Users(users))
}
