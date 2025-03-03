use serde::{Deserialize, Serialize};
use std::time::Duration;
use serde_with::{serde_as, DurationSeconds};
use thiserror::Error;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
  #[serde_as(as = "DurationSeconds<String>")]
  pub timeout: Duration,
  pub proxy: Option<String>,
  #[serde(rename = "useProxy")]
  pub use_proxy: Option<bool>,
}

#[derive(Error, Debug, Serialize)]
pub enum AppError {
  #[error("error: {wrapped}")]
  WrapError{wrapped: String}
}