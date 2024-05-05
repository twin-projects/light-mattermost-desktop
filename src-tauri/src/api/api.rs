use reqwest::{Client, StatusCode};
use url::Url;

use crate::api::call_event::{ApiEvent, LoginRequest, Response, UserResponse};

static API_VERSION: &str = "/api/v4/";

pub async fn handle_request(
    client: &Client,
    server_url: &Url,
    event: &ApiEvent,
) -> Result<Response, crate::Error> {
    match &event {
        ApiEvent::LoginEvent(login, password) => {
            let login_request = LoginRequest {
                login_id: (&login).to_string(),
                password: (&password).to_string(),
            };
            let payload = serde_json::to_string(&login_request).unwrap();
            let uri = server_url.join(API_VERSION)?.join("users/login")?;
            tracing::info!("call: {uri} payload: {payload}");
            let response = client.post(uri).body(payload).send().await.unwrap();
            match &response.status() {
                &StatusCode::OK => {
                    let login_from_header: &str =
                        &response.headers().get("token").unwrap().to_str().unwrap();
                    let token: String = login_from_header.to_owned();
                    let user: UserResponse = response.json::<UserResponse>().await.unwrap();
                    Ok(Response::LoginResponse(token, user))
                }
                _ => Err(crate::Error::PoisonError(
                    "MatterMost server rejected request".into(),
                )),
            }
        }
    }
}

