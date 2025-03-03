use tokio::sync::Mutex;
use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
pub mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::TranslatorBindings;
#[cfg(mobile)]
use mobile::TranslatorBindings;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the translator-bindings APIs.
pub trait TranslatorBindingsExt<R: Runtime> {
  fn translator_bindings(&self) -> &Mutex<TranslatorBindings<R>>;
}

impl<R: Runtime, T: Manager<R>> crate::TranslatorBindingsExt<R> for T {
  fn translator_bindings(&self) -> &Mutex<TranslatorBindings<R>> {
    self.state::<Mutex<TranslatorBindings<R>>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::<R>::new("translator-bindings")
    .invoke_handler(tauri::generate_handler![commands::translate])
    .setup(|app, api| {
      #[cfg(mobile)]
      let translator_bindings = mobile::init(app, api)?;
      #[cfg(desktop)]
      let translator_bindings = Mutex::new(desktop::init(app, api)?);
      app.manage(translator_bindings);
      Ok(())
    })
    .build()
}
