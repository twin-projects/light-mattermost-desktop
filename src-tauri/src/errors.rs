#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Failed to read credentials: {_0}")]
    Zbox(#[from] zbox::Error),
    #[error("Failed to write credentials: {_0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to deserialize credentials: {_0}")]
    De(#[from] bincode::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum NativeError {
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
pub enum Error {
    #[error(transparent)]
    Native(#[from] NativeError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Standard(#[from] core::fmt::Error),
    #[error("the mutex was poisoned")]
    PoisonError(String),
    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error(transparent)]
    Storage(#[from] StorageError),
}
