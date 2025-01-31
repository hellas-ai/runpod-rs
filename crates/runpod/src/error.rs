use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunpodError {
    #[error("API request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Rate limited")]
    RateLimited,

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] serde_json::Error),

    #[error("URL Parsing Error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Parse Error: {0}")]
    GraphQLError(String),

    #[error("Toml Error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
}

impl From<graphql_client::Error> for RunpodError {
    fn from(err: graphql_client::Error) -> Self {
        RunpodError::GraphQLError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, RunpodError>;
