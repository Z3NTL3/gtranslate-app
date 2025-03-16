// #![allow(unused)]
// use crate::models;

// We might prefer streamlined channel communication between the Rust core and GUI but for now 
// event based communication didn't really have noticable problems in the smoothness of the update progress
//
// #[tauri::command]
// pub async fn update_application(streamline: tauri::ipc::Channel<models::AppPayload<'_>>, handle: tauri::AppHandle) -> Result<String, ()> {
//   todo!()
// }