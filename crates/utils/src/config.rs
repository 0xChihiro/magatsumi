use super::errors::MagatsumiError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::result::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {}

impl Config {
    pub fn open(path: PathBuf) -> Result<Self, MagatsumiError> {
        let contents = fs::read_to_string(&path)
            .map_err(|_| MagatsumiError::ConfigNotFound(path.display().to_string()))?;
        let config =
            serde_json::from_str(&contents).map_err(|source| MagatsumiError::ConfigInvalid {
                path: path.display().to_string(),
                source,
            })?;
        Ok(config)
    }

    pub fn save(&self, path: PathBuf) -> Result<(), MagatsumiError> {
        let contents = serde_json::to_string_pretty(self).map_err(|source| {
            MagatsumiError::ConfigSaveFailed {
                path: path.display().to_string(),
                reason: source.to_string(),
            }
        })?;
        fs::write(&path, contents).map_err(|source| MagatsumiError::ConfigSaveFailed {
            path: path.display().to_string(),
            reason: source.to_string(),
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(filename: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        let mut path = std::env::temp_dir();
        path.push(format!("magatsumi-{}-{}", nanos, filename));
        path
    }

    #[test]
    fn open_returns_not_found_error() {
        let path = temp_path("missing.json");
        let err = Config::open(path).expect_err("expected error for missing file");
        match err {
            MagatsumiError::ConfigNotFound(_) => {}
            other => panic!("expected ConfigNotFound, got {other:?}"),
        }
    }

    #[test]
    fn open_returns_invalid_error() {
        let path = temp_path("invalid.json");
        fs::write(&path, "not json").expect("write invalid json");
        let err = Config::open(path.clone()).expect_err("expected error for invalid json");
        match err {
            MagatsumiError::ConfigInvalid { path: err_path, .. } => {
                assert_eq!(err_path, path.display().to_string());
            }
            other => panic!("expected ConfigInvalid, got {other:?}"),
        }
        let _ = fs::remove_file(path);
    }

    #[test]
    fn save_writes_config_file() {
        let path = temp_path("save.json");
        let config = Config {};
        config.save(path.clone()).expect("save config");

        let read_back = fs::read_to_string(&path).expect("read saved config");
        assert!(!read_back.is_empty());
        let _ = Config::open(path.clone()).expect("open saved config");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn save_returns_error_when_directory_missing() {
        let mut path = temp_path("missing-dir");
        path.push("config.json");
        let config = Config {};
        let err = config.save(path.clone()).expect_err("expected save error");
        match err {
            MagatsumiError::ConfigSaveFailed { path: err_path, .. } => {
                assert_eq!(err_path, path.display().to_string());
            }
            other => panic!("expected ConfigSaveFailed, got {other:?}"),
        }
    }
}
