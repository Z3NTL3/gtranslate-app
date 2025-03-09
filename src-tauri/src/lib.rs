use tauri::{
    async_runtime::{self, JoinHandle},
    menu::{MenuBuilder, MenuItemBuilder},
    path::BaseDirectory,
    tray::TrayIconBuilder,
    Emitter, Listener, Manager,
};
use tauri_plugin_positioner::{Position, WindowExt};
use tauri_plugin_translator_bindings::TranslatorBindingsExt;
use tauri_plugin_updater::UpdaterExt;
use tracing_subscriber::{filter::LevelFilter, fmt::time::ChronoLocal};

#[cfg(desktop)]
mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused)]
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|mut app| {
            let handle = app.handle().clone();
           let _: JoinHandle<tauri_plugin_updater::Result<()>> = async_runtime::spawn(async move {
                if let Some(update) = handle.updater()?.check().await? {
                    let mut downloaded = 0;
        
                    update
                        .download_and_install(
                        |chunk_length, content_length| {
                            downloaded += chunk_length;
                            println!("downloaded {downloaded} from {content_length:?}");
                        },
                        || {
                            println!("download finished");
                        },
                        )
                        .await?;
                
                    println!("update installed");
                    handle.restart();
                }
                
                Ok(())
            });
           
            let handle = app.handle().clone();
            app.listen(models::EXIT, move |_| handle.exit(0));

            let handle = app.handle().clone();
            app.listen(models::WINDOW_LOADED, move |event| {
                println!("event: {}", event.payload());
                for (label, window) in &handle.webview_windows() {
                    if !label.contains("main") {
                        if let None = window.is_focused().is_ok().then(|| {
                            println!("window show");
                            window.show();
                        }) {
                            window.is_visible().is_ok().then(|| {
                                println!("window show");
                                window.show();
                            });
                        }
                    }
                }
            });

            // test if our app is opened with required privileges
            // because we need more permissions on some systems to write files in our app sitting within directories requiring higher permissions
            let test_file = app
                .path()
                .resolve("test.log", BaseDirectory::Resource)
                .unwrap();

            let mut file_opener = tokio::fs::OpenOptions::new();
            file_opener.read(true).append(true).write(true).create(true);

            let test_file_ = test_file.clone();
            let handle = app.handle();
            let exit = async_runtime::block_on(async move {
                match file_opener.open(test_file_).await {
                    Ok(_) => false,
                    Err(err) => {
                        let window = tauri::WebviewWindowBuilder::new(
                            handle,
                            "app-failures",
                            tauri::WebviewUrl::App("src/failures.html".into()),
                        )
                        .center()
                        .closable(false)
                        .resizable(false)
                        .decorations(false)
                        .minimizable(false)
                        .maximizable(false)
                        .inner_size(300.0, 200.0)
                        .visible(false)
                        .always_on_top(true)
                        .build()
                        .unwrap();

                        true
                    }
                }
            });

            println!("tokio rm file");
            async_runtime::block_on(async move {
                tokio::fs::remove_file(test_file).await;
            });
            println!("should be removed");

            if exit {
                return Ok(());
            }

            let log_file = app
                .path()
                .resolve("app.log", BaseDirectory::Resource)
                .unwrap();
            let tracing = tracing_appender::rolling::never(log_file, "app.log");

            tracing_subscriber::fmt()
                .with_writer(tracing)
                .with_max_level(LevelFilter::DEBUG)
                .with_timer(ChronoLocal::new("%v - %H:%M:%S".to_owned()))
                .with_file(true)
                .with_line_number(true)
                .json()
                .with_current_span(true)
                .init();

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
                        .on_tray_icon_event(|icon, event| {
                            tauri_plugin_positioner::on_tray_event(icon.app_handle(), &event);
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
                        .unwrap();
                });
            }

            // read app config and deserialize it; some todo's todo
            let app_config = app
                .path()
                .resolve("app-conf.json", BaseDirectory::Resource)
                .map_err(|err| {
                    tracing::error!("could not resolve app-conf.json: {}", err);
                    err
                })?;
            let file = std::fs::File::open(&app_config).map_err(|err| {
                tracing::error!("failed opening config file: {}", err);
                err
            })?;

            let config: tauri_plugin_translator_bindings::AppConfig =
                serde_json::from_reader(file)?;

            #[cfg(desktop)]
            {
                use tauri_plugin_autostart::MacosLauncher;
                use tauri_plugin_autostart::ManagerExt;

                app.handle().plugin(tauri_plugin_autostart::init(
                    MacosLauncher::LaunchAgent,
                    None,
                ));

                let launch = app.autolaunch();
                if let Some(_) = config.autostart {
                    launch.enable();
                } else {
                    if launch.is_enabled()? {
                        launch.disable();
                    }
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
                        .expect("failed setting proxy");
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
