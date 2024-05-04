// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::error;
use serde::Serialize;
use tauri::{State};
use tokio::sync::Mutex;

use api::handle_request;

use crate::api::call_event::{ApiEvent, Response, UserResponse};

mod api;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Standard(#[from] core::fmt::Error),
    #[error("the mutex was poisoned")]
    PoisonError(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::ser::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        Error::PoisonError(err.to_string())
    }
}

#[derive(Serialize, Clone)]
pub(crate) struct AuthState {
    #[serde(skip_serializing)]
    token: Option<String>,
    logged_in: bool,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            token: None,
            logged_in: false,
        }
    }
}

#[tauri::command]
async fn login(login: String, password: String, state_mutex: State<'_, Mutex<AuthState>>) -> Result<UserResponse, Error> {
    println!("Logging in");
    let mut state = state_mutex.lock().await;
    let response = handle_request(&ApiEvent::LoginEvent(login, password)).await;
    match response {
        Ok(data) => {
            match data {
                Response::LoginResponse(token, user) => {
                    state.token = Some(token);
                    Ok(user)
                }
            }
        }
        Err(error) => Err(Error::Standard(error))
    }
    // Ok(state.clone())
}

#[tauri::command]
async fn greet(name: &str) -> Result<String, ()> {
    let _response = handle_request(
        &ApiEvent::LoginEvent("admin".to_owned(), "admin123!".to_owned())
    ).await;
    Ok(format!("Hello {} from Rust!", name))
}


#[tokio::main]
async fn main() {
    tauri::Builder::default()
        // .manage(Mutex::new(Api::instance()))
        .manage(Mutex::new(AuthState::default()))
        .invoke_handler(tauri::generate_handler![greet, login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
