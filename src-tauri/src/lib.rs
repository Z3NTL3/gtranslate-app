use std::time::Duration;

use tauri::{
    async_runtime::{self, JoinHandle}, menu::{MenuBuilder, MenuItemBuilder}, path::BaseDirectory, tray::TrayIconBuilder, window::Color, Emitter, Listener, Manager, WebviewWindowBuilder
};
use tauri_plugin_positioner::{Position, WindowExt};
use tauri_plugin_translator_bindings::TranslatorBindingsExt;
use tauri_plugin_updater::UpdaterExt;
use tracing_subscriber::{filter::LevelFilter, fmt::time::ChronoLocal};

#[cfg(desktop)]
mod models;

pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused)]
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_positioner::init())
        .setup(|mut app| {
            // better tokio tracing support coming on patch v3.2.0
            tracing_subscriber::fmt()
                .with_max_level(LevelFilter::ERROR)
                .with_timer(ChronoLocal::new("%v - %H:%M:%S".to_owned()))
                .with_file(true)
                .with_line_number(true)
                .json()
                .with_current_span(true)
                .init();

            // setup some important handlers
            let handle = app.handle().clone();
            app.listen(models::EXIT, move |_| handle.exit(0));

            let handle = app.handle().clone();
            app.listen(models::WINDOW_LOADED, move |event| {
                for (label, window) in &handle.webview_windows() {
                    if !label.contains("main") {
                        window.show();
                    }
                }
            });
    
            // self-updater
            let handle = app.handle().clone();
            let _: JoinHandle<tauri_plugin_updater::Result<()>> = async_runtime::spawn(async move {
                if let Some(update) = handle.updater()?.check().await? {
                    println!("update found");

                    // create updater window
                    let window = WebviewWindowBuilder::new(&handle, "updater", tauri::WebviewUrl::App("src/updater.html".into()))
                        .always_on_top(true)
                        .decorations(false)
                        .devtools(false)
                        .inner_size(400 as f64, 400 as f64)
                        .minimizable(false)
                        .maximizable(false)
                        .closable(false)
                        .visible(false)
                        .center()
                        .background_color(Color(34, 34, 34, 1))
                        .build()?;

                    window.hide();

                    // hide main window
                    handle.get_webview_window("main")
                        .expect("failed getting main window")
                        .hide();

                    let mut downloaded = 0;
                    let file_contents = update.download(
                        |chunk_length, content_length| {
                            downloaded += chunk_length;
                            handle.emit(models::UPDATE_PROGRESS, models::AppPayload{
                                identifier: "update",
                                message: &format!("{}/{}", downloaded, content_length.unwrap_or_default())
                            });
                        },
                        || {
                            handle.emit(models::UPDATE_COMPLETED, models::AppPayload{
                                identifier: "completed",
                                message: "update completed"
                            });
                        },
                    ).await?;
                    
                    std::thread::sleep(Duration::from_secs(2)); // sleep on main thread
                    update.install(file_contents);
                    
                    handle.restart();
                }
                Ok(())
            });

            // build and configure system tray stuff in background
            #[cfg(desktop)]
            {
                let open_item = MenuItemBuilder::new("Open").id("open").build(app)?;
                let quit_item = MenuItemBuilder::new("Quit").id("quit").build(app)?;

                let menu = MenuBuilder::new(app)
                    .item(&open_item)
                    .item(&quit_item)
                    .build()
                    .map_err(|e| {
                        tracing::error!("{e}");
                        e
                    })?;

                let handle = app.handle().clone();
                async_runtime::spawn(async move {
                    let tray = TrayIconBuilder::new()
                        .icon(handle.default_window_icon().unwrap().clone())
                        .menu(&menu)
                        .show_menu_on_left_click(false)
                        .on_tray_icon_event(|icon, event| {
                            let handle = icon.app_handle();
                            // add support for feat: open directly on double click in systems tray / app bar
                            let dbl_click: bool = {
                                if let Some(window) = handle.get_webview_window("main") {
                                     match &event {
                                        // Windows only compability
                                        #[cfg(target_os = "windows")]
                                        tauri::tray::TrayIconEvent::DoubleClick { .. } => {
                                            window.as_ref().window().move_window_constrained(Position::TrayBottomRight);
                                            window.show();

                                            handle.emit(
                                                models::START_GLOW_EFFECT,
                                                models::AppPayload {
                                                    identifier: "info",
                                                    message: "start glow effect",
                                                },
                                            );
                                            true
                                        },

                                        // This should only be valid on MacOs and Linux
                                        // On Windows this might have irritated some users when trying to Quit from the system tray
                                        #[cfg(any(target_os = "macos", target_os = "linux"))]
                                        tauri::tray::TrayIconEvent::Click { .. } => {
                                            window.as_ref().window().move_window_constrained(Position::TrayBottomRight);
                                            window.show();

                                            handle.emit(
                                                models::START_GLOW_EFFECT,
                                                models::AppPayload {
                                                    identifier: "info",
                                                    message: "start glow effect",
                                                },
                                            );
                                            true
                                        }
                                        _ => false,
                                    };
                                }
                                false
                            };

                            if !dbl_click {
                                return tauri_plugin_positioner::on_tray_event(icon.app_handle(), &event);
                            } 
                        })
                        .on_menu_event(|app, event| match event.id.as_ref() {
                            "open" => {
                                if let Some(window) = app.get_webview_window("main") {
                                    window
                                        .as_ref()
                                        .window()
                                        .move_window_constrained(Position::TrayBottomRight);
                                    window.show();
                                    window.set_focus();

                                    app.emit(
                                        models::START_GLOW_EFFECT,
                                        models::AppPayload {
                                            identifier: "info",
                                            message: "start glow effect",
                                        },
                                    );
                                }
                            }
                            "quit" => app.exit(0),
                            _ => (),
                        })
                        .build(&handle)
                        .map_err(|e| {
                            tracing::error!("{e}");
                            e
                        })
                        .unwrap();
                });
            }

            // read app config and deserialize it; but it should not panic
            let app_config = app
                .path()
                .resolve("app-conf.json", BaseDirectory::Resource)
                .map_err(|err| {
                    tracing::error!("could not resolve app-conf.json: {}", err);
                    err
                });

            if let Err(e) = app_config {
                tracing::error!("err while trying to read app config: {e}");
                return Ok(());
            }  

            let app_config = app_config.unwrap();

            let file = std::fs::File::open(&app_config).map_err(|err| {
                tracing::error!("failed opening config file: {}", err);
                err
            });

            if let Err(e) = file {
                tracing::error!("err while opening config file (resource) in read mode: {e}");
                return Ok(());
            }

            let file = file.unwrap();
            let config: Result<tauri_plugin_translator_bindings::AppConfig, serde_json::Error> =
                serde_json::from_reader(file);

            if let Err(e) = config {
                tracing::error!("err while opening config file (resource) in read mode: {e}");
                return Ok(());
            }

            let config = config.unwrap();

            #[cfg(desktop)]
            {
                use tauri_plugin_autostart::MacosLauncher;
                use tauri_plugin_autostart::ManagerExt;

                app.handle().plugin(tauri_plugin_autostart::init(
                    MacosLauncher::LaunchAgent,
                    None,
                ));

                let launch = app.autolaunch();
                if let Some(should_enable) = config.autostart {
                    if !should_enable {
                        launch.disable();
                    } else {
                        launch.enable();
                    }
                   
                } else {
                    launch.enable();
                }
            }

            if config.use_proxy.is_some_and(|v| v == true) {
                let handle = app.handle().clone();
                tauri::async_runtime::block_on(async move {
                    handle
                        .translator_bindings()
                        .lock()
                        .await
                        .use_proxy(
                            config
                                .proxy
                                .expect("use_proxy is set to `true`, `proxy` must be set as well"),
                        )
                        .map_err(|e| {
                            tracing::error!("{e}");
                            e
                        })
                });
            }
            Ok(())
        })
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_translator_bindings::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
