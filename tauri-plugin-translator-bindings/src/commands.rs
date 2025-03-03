#![allow(unused)]
use std::time::Duration;
use tauri::Manager;
use tauri::{AppHandle, command, Runtime};

use crate::desktop::TranslatorBindings;
use crate::models::*;
use crate::TranslatorBindingsExt;

#[tauri::command(rename_all = "snake_case")]
pub async fn translate(source_lang: &'static str, target_lang: &'static str, query: &'static str, handle: tauri::AppHandle) -> Result<(), AppError> {
    let translated= handle.translator_bindings().lock().await.translator.translate(
        Duration::from_secs(5), // todo
        gtranslate::TranslateOptions::new()
            .set_source_lang(source_lang)
            .set_target_lang(target_lang)
            .query(query)
    ).await.map_err(|err| {
        AppError::WrapError { wrapped: err.to_string() }
    })?;

    // todo stuff
    println!("{translated}");
    Ok(())
}
