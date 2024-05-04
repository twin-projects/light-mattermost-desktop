use std::fmt::Error;

use reqwest::{Client, StatusCode};

use crate::api::call_event::{ApiEvent, LoginRequest, Response, UserResponse};

pub async fn handle_request(event: &ApiEvent) -> Result<Response, Error> {
    let client: Client = Client::new();
    let url_base: &str = "http://localhost:8065/api/v4/";
    match &event {
        ApiEvent::LoginEvent(login, password) => {
            let login_request = LoginRequest {
                login_id: (&login).to_string(),
                password: (&password).to_string(),
            };
            let payload = serde_json::to_string(&login_request).unwrap();
            let response = client
                .post(format!("{url_base}users/login"))
                .body(payload)
                .send()
                .await
                .unwrap();
            println!("response: {response:#?}");
            match &response.status() {
                &StatusCode::OK => {
                    let login_from_header: &str = &response.headers().get("token").unwrap().to_str().unwrap();
                    let token: String = login_from_header.to_owned();
                    let user: UserResponse = response.json::<UserResponse>().await.unwrap();
                    Ok(Response::LoginResponse(token, user))
                }
                _ => Err(Default::default()),
            }
        }
    }
}