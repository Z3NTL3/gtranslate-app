#![allow(unused)]
use std::time::Duration;
use tauri::Manager;
use tauri::{AppHandle, command, Runtime};

use crate::desktop::TranslatorBindings;
use crate::models::{TranslationOpts, AppError};
use crate::TranslatorBindingsExt;

#[tauri::command(rename_all = "snake_case")]
pub async fn translate<R: Runtime>(translations_options: TranslationOpts<'_>, app_handle: AppHandle<R>) -> Result<(), AppError> {
    println!("opts: {:?}", translations_options);
    let translated= app_handle.translator_bindings().lock().await.translator.translate(
        Duration::from_secs(5), // todo
        translations_options.opts
    ).await.map_err(|err| {
        AppError::WrapError { wrapped: err.to_string() }
    })?;

    // todo stuff
    println!("translated: {translated}");
    Ok(())
}
