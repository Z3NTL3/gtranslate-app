use tauri::{path::BaseDirectory, Manager,};
use tauri_plugin_translator_bindings::TranslatorBindingsExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused)]
    tauri::Builder::default()
        .setup(|app| {
            let app_config = app.path().resolve("../src/assets/app-conf.json", BaseDirectory::Resource)?;
            let file = std::fs::File::open(&app_config)?;

            let config: tauri_plugin_translator_bindings::AppConfig = serde_json::from_reader(file)?; 
            if config.use_proxy.is_some_and(|v| {
                v == true
            }){
                tauri::async_runtime::block_on(async move {
                    app.translator_bindings()
                        .lock()
                        .await
                        .use_proxy(config.proxy.expect("use_proxy is set to `true`, `proxy` must be set as well"))
                        .expect("failed setting proxy");
                });
            }
               
            Ok(())
        })
        // .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_translator_bindings::init())    
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
