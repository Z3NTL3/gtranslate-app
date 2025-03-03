#![allow(unused)]
use std::time::Duration;
use tauri::Manager;
use tauri::{AppHandle, command, Runtime};

use crate::desktop::TranslatorBindings;
use crate::models::{TranslationOpts, AppError};
use crate::TranslatorBindingsExt;

#[tauri::command(rename_all = "snake_case")]
pub async fn translate<R: Runtime>(timeout_secs: u64, translations_options: TranslationOpts<'_>, app_handle: AppHandle<R>) -> Result<String, AppError> {
    let translated= app_handle.translator_bindings().lock().await.translator.translate(
        Duration::from_secs(timeout_secs), // todo
        translations_options.opts
    ).await.map_err(|err| {
        AppError::WrapError { wrapped: err.to_string() }
    })?;

    Ok(translated)
}
