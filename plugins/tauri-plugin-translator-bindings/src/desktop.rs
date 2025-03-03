#![allow(unused)]
use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Manager, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<TranslatorBindings<R>> {
  Ok(TranslatorBindings{translator: gtranslate::Translator::new(), handle: app.clone()})
}

/// Access to the translator-bindings APIs.
pub struct TranslatorBindings<R: Runtime>{
  pub translator: gtranslate::Translator,
  handle: AppHandle<R>
}

impl<R: Runtime> TranslatorBindings<R> {
  /// Forces the client to use the given proxy
  pub fn use_proxy(&mut self, proxy: String) -> Result<(), Box<dyn std::error::Error>>{
    self.translator = gtranslate::Translator::with_client(
      reqwest::Client::builder()
        .proxy(reqwest::Proxy::all(proxy)?)
        .build()?
    );
    Ok(())
  }
}
