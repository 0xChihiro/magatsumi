use thiserror::Error;

#[derive(Debug, Error)]
pub enum MagatsumiError {
    #[error("unable to find config file at {0}")]
    ConfigNotFound(String),
    #[error("unable to parse config file at {path}: {source}")]
    ConfigInvalid {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("unable to save config file at {path}: {reason}")]
    ConfigSaveFailed { path: String, reason: String },
}
