use std::sync::Mutex;
use tauri::{
    async_runtime,
    menu::{MenuBuilder, MenuItemBuilder},
    path::BaseDirectory,
    tray::TrayIconBuilder,
    Manager,
};
use tauri_plugin_translator_bindings::TranslatorBindingsExt;

#[cfg(desktop)]
mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused)]
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|mut app| {
            // this doesnt work as well visible: false has no effect either
            let main = app.get_webview_window("main")
                .expect("failed getting main window");

            main.hide();
            app.manage(Mutex::new(models::AppData {
                is_position_set: false,
            }));

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
                            let handle = icon.app_handle();
                            let main = handle
                                .get_webview_window("main")
                                .expect("couldn't get mai window");

                            let state = handle.state::<Mutex<models::AppData>>();
                            let mut data = state.lock().expect("failed locking main thread");

                            println!("{:?}{:?}", icon.id().0, data.is_position_set);

                            if !data.is_position_set {
                                match event {
                                    tauri::tray::TrayIconEvent::Click {
                                        id,
                                        position,
                                        rect,
                                        button,
                                        button_state,
                                    } => {
                                        main.set_position(position);
                                        data.is_position_set = true;
                                    }
                                    tauri::tray::TrayIconEvent::DoubleClick {
                                        id,
                                        position,
                                        rect,
                                        button,
                                    } => {
                                        main.set_position(position);
                                        data.is_position_set = true;
                                    }
                                    tauri::tray::TrayIconEvent::Enter { id, position, rect } => {
                                        main.set_position(position);
                                        data.is_position_set = true;
                                    },
                                    tauri::tray::TrayIconEvent::Move { id, position, rect } => (),
                                    tauri::tray::TrayIconEvent::Leave { id, position, rect } => (),
                                    _ => todo!(),
                                }
                            }
                        })
                        .on_menu_event(|app, event| match event.id.as_ref() {
                            "open" => {
                                if let Some(window) = app.get_webview_window("main") {
                                    window.show();
                                    window.set_focus();
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
                .resolve("../src/assets/app-conf.json", BaseDirectory::Resource)?;
            let file = std::fs::File::open(&app_config)?;

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
        // .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_translator_bindings::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
