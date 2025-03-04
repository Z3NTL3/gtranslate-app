use tauri::{
    async_runtime, menu::{MenuBuilder, MenuItemBuilder}, path::BaseDirectory, tray::TrayIconBuilder, Emitter, Manager
};
use tauri_plugin_translator_bindings::TranslatorBindingsExt;

#[cfg(desktop)]
mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused)]
    tauri::Builder::default()
        .setup(|mut app| {
            // build and configure system tray stuff in background
            #[cfg(desktop)]
            {
                let open_item = MenuItemBuilder::new("Open").id("open").build(app)?;
                let quit_item = MenuItemBuilder::new("Quit").id("quit").build(app)?;

                let menu = MenuBuilder::new(app)
                    .item(&open_item)
                    .item(&quit_item)
                    .build()?;
      
                let handle = app.handle().clone();
                async_runtime::spawn(async move {
                    let tray = TrayIconBuilder::new()
                    .icon(handle.default_window_icon().unwrap().clone())
                    .menu(&menu)
                    .on_menu_event(|app, event| match event.id.as_ref() {
                        "open" => {
                            if let Some(window) = app.get_webview_window("main") {
                                window.set_focus();
                                window.hide_menu();
                            }
                        }
                        "quit" => app.exit(0),
                        _ => (),
                    })
                    .build(&handle).unwrap();
                });
                
            }

            // read app config and deserialize it; some todo's todo
            let app_config = app
                .path()
                .resolve("../src/assets/app-conf.json", BaseDirectory::Resource)?;
            let file = std::fs::File::open(&app_config)?;

            let config: tauri_plugin_translator_bindings::AppConfig =
                serde_json::from_reader(file)?;

            if config.use_proxy.is_some_and(|v| v == true) {
                app = tauri::async_runtime::block_on(async move {
                    app.translator_bindings()
                        .lock()
                        .await
                        .use_proxy(
                            config
                                .proxy
                                .expect("use_proxy is set to `true`, `proxy` must be set as well"),
                        )
                        .expect("failed setting proxy");
                    app
                });
            }

            // configure shortcut keybind to run in background
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                };

                let handle = app.handle().clone();
                async_runtime::spawn(async move {
                    let ctrl_t = Shortcut::new(Some(Modifiers::ALT), Code::KeyT);
                    handle.plugin(
                        tauri_plugin_global_shortcut::Builder::new()
                            .with_handler(move |_app, shortcut, event| {
                                if shortcut == &ctrl_t {
                                    match event.state() {
                                        ShortcutState::Pressed => {
                                            let main_window = _app.get_webview_window("main");
                                            if let None = main_window {
                                                _app.emit(
                                                    models::WINDOW_RETRIEVAL_FAILURE, 
                                                    models::AppPayload{
                                                        identifier: "error",
                                                        message: "failed retrieving main window"
                                                    }
                                                );    
                                            } else {
                                                let main_window = main_window.expect("failed retrieving main window");

                                                if let Ok(_) = main_window.is_visible() {
                                                    main_window.hide();
                                                } else {
                                                    main_window.show();
                                                    main_window.set_focus();
                                                }
                                            }
                                        }
                                        ShortcutState::Released => (),
                                    }
                                }
                            })
                            .build(),
                    ).expect("failed initializing shortcut plugin");
                    handle.global_shortcut().register(ctrl_t)
                        .expect("failed registering shortcut plugin");
                })
            };
            Ok(())
        })
        // .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_translator_bindings::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
