use tauri::{menu::{MenuBuilder, MenuItemBuilder}, path::BaseDirectory, tray::TrayIconBuilder, Emitter, Manager};
use tauri_plugin_translator_bindings::TranslatorBindingsExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused)]
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|mut app| {
            #[cfg(desktop)] 
            {
                let open_item = MenuItemBuilder::new("Open")
                    .id("open")
                    .build(app)?;

                let quit_item = MenuItemBuilder::new("Quit")
                    .id("Quit")
                    .build(app)?;

                let menu = MenuBuilder::new(app)
                    .item(&open_item)
                    .item(&quit_item)
                    .build()?;

                let tray = TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&menu)
                    .on_menu_event(|app, event| 
                        match event.id.as_ref() {
                            "open" => {
                                if let Some(window) = app.get_webview_window("main") {
                                    window.set_focus();
                                }
                            }
                            "quit" => app.exit(0),
                            _ => ()
                        }
                    )
                    .build(app)?;
            }

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

            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

                let ctrl_t = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyT);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler( move |_app, shortcut, event| {
                            println!("{:?}", shortcut);
                            if shortcut == &ctrl_t {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        let main_window = _app
                                            .get_webview_window("main")
                                            .unwrap(); // for now todo change!

                                       // if let None = main_window {
                                            // app.emit("window_retrieval_failure", payload); // todo: ;``event`` an Enum resolving to &str
                                        // }
                                        
                                        
                                        if let Ok(_) = main_window.is_visible() {
                                            main_window.set_focus();
                                        } else {
                                            main_window.hide();
                                        }
                                        
                                    }
                                    ShortcutState::Released => ()
                                }
                            }
                        })
                    .build()
                )?;
                app.global_shortcut().register(ctrl_t)?;
            };

            Ok(())
        })
        // .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_translator_bindings::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
