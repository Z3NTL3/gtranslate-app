use serde::{Deserialize, Serialize};

#[allow(unused)]
pub const WINDOW_RETRIEVAL_FAILURE: &'static str = "window_retrieval_failure";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AppPayload<'a> {
    pub identifier: &'a str,
    pub message: &'a str,
}

pub struct AppData {
    pub is_position_set: bool,
}
