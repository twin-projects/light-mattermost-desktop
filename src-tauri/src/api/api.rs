use reqwest::header::HeaderMap;
use reqwest::{Client, Method};
use serde::Serialize;
use url::Url;

use crate::api::call_event::*;
use crate::errors::*;

macro_rules! join_url {
    ($url: expr, $($piece: expr),*) => {
       $url$(.join($piece).expect("url must be valid"))*
    };
}

pub async fn handle_request(
    client: &Client,
    server_url: &Url,
    event: &ApiEvent,
    token: Option<&String>,
) -> Result<Response, Error> {
    let server_url = join_url!(server_url, "api", "v4");

    match event {
        ApiEvent::LoginEvent(login_id, password) => {
            login(
                client,
                join_url!(server_url, "users", "login"),
                &login_id,
                &password,
            )
            .await
        }
        ApiEvent::MyTeams => {
            my_teams(client, join_url!(server_url, "users", "me", "teams"), token).await
        }
    }
}

async fn handle<T: Serialize>(
    client: &Client,
    method: Method,
    url: Url,
    payload: Option<T>,
    token: Option<&String>,
) -> reqwest::Response {
    let mut builder = client.request(method, url);
    builder = match payload {
        Some(json) => builder.json(&json),
        _ => builder,
    };
    builder = match token {
        Some(bearer_token) => builder.bearer_auth(format!("Bearer {bearer_token}")),
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
    let login_request = LoginRequest {
        login_id: login.to_string(),
        password: password.to_string(),
    };
    let response = handle(client, Method::POST, uri, Some(login_request), None).await;
    if !response.status().is_success() {
        return Err(NativeError::PerformLogin)?;
    }
    let token = get_token(&response.headers()).to_owned();
    let user_response = &response.json::<UserResponse>().await;
    match user_response {
        Ok(user) => {
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

fn get_token<'h>(headers: &'h HeaderMap) -> &'h str {
    headers
        .get("token")
        .and_then(|header| header.to_str().ok())
        .unwrap_or_default()
}

async fn my_teams(client: &Client, uri: Url, token: Option<&String>) -> Result<Response, Error> {
    let response = handle(client, Method::GET, uri, None as Option<()>, token).await;
    if response.status().is_success() {
        let teams: Vec<Team> = response.json::<Vec<Team>>().await.unwrap();
        Ok(Response::MyTeams(teams))
    } else {
        Err(NativeError::FetchTeams)?
    }
}
