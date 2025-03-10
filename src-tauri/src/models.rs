#![allow(unused)]
use serde::{Deserialize, Serialize};

pub const START_GLOW_EFFECT: &'static str = "start_glow_effect";
pub const WINDOW_LOADED: &'static str = "window_loaded";
pub const EXIT: &'static str = "exit";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AppPayload<'a> {
    pub identifier: &'a str,
    pub message: &'a str,
}
