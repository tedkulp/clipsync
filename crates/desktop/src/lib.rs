mod clipboard;
mod config;
mod sync;

use std::sync::Arc;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, State, WindowEvent};
use tokio::sync::RwLock;

use crate::config::Config;
use crate::sync::SyncManager;

pub struct AppState {
    sync_manager: Arc<RwLock<SyncManager>>,
    config: Arc<RwLock<Config>>,
}

#[tauri::command]
async fn connect_to_server(
    server_url: String,
    shared_secret: String,
    state: State<'_, Arc<RwLock<AppState>>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    tracing::info!("Connect command called with URL: {}", server_url);

    let app_state = state.read().await;

    // Save config
    {
        let mut config = app_state.config.write().await;
        config.server_url = Some(server_url.clone());
        config.shared_secret = Some(shared_secret.clone());
        config.save().map_err(|e| {
            tracing::error!("Failed to save config: {}", e);
            e.to_string()
        })?;
    }

    // Connect
    let mut sync_manager = app_state.sync_manager.write().await;
    sync_manager
        .connect(server_url, shared_secret, app)
        .await
        .map_err(|e| {
            tracing::error!("Connection failed: {}", e);
            e.to_string()
        })?;

    tracing::info!("Successfully connected");
    Ok(())
}

#[tauri::command]
async fn disconnect_from_server(state: State<'_, Arc<RwLock<AppState>>>) -> Result<(), String> {
    let app_state = state.read().await;
    let mut sync_manager = app_state.sync_manager.write().await;
    sync_manager.disconnect().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn toggle_sync(paused: bool, state: State<'_, Arc<RwLock<AppState>>>) -> Result<(), String> {
    let app_state = state.read().await;
    let mut sync_manager = app_state.sync_manager.write().await;
    sync_manager.set_paused(paused);
    Ok(())
}

#[tauri::command]
async fn get_config(state: State<'_, Arc<RwLock<AppState>>>) -> Result<Config, String> {
    let app_state = state.read().await;
    let config = app_state.config.read().await;
    Ok(config.clone())
}

#[tauri::command]
async fn show_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn hide_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn set_autostart(
    enabled: bool,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<(), String> {
    let app_state = state.read().await;
    let mut config = app_state.config.write().await;
    config.autostart = enabled;
    config.save().map_err(|e| e.to_string())?;

    // TODO: Implement actual autostart registration per platform
    // For now, just save the preference
    Ok(())
}

#[tauri::command]
async fn set_start_minimized(
    enabled: bool,
    state: State<'_, Arc<RwLock<AppState>>>,
) -> Result<(), String> {
    let app_state = state.read().await;
    let mut config = app_state.config.write().await;
    config.start_minimized = enabled;
    config.save().map_err(|e| e.to_string())?;
    Ok(())
}

fn create_tray_menu(app: &AppHandle) -> Result<Menu<tauri::Wry>, tauri::Error> {
    let show_item = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
    let hide_item = MenuItem::with_id(app, "hide", "Hide Window", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    Menu::with_items(app, &[&show_item, &hide_item, &quit_item])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "clipsync_desktop=debug".into()),
        )
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Load config
            let config = Config::load().unwrap_or_default();
            let start_minimized = config.start_minimized;

            // Create sync manager
            let sync_manager = SyncManager::new();

            // Create app state
            let app_state = Arc::new(RwLock::new(AppState {
                sync_manager: Arc::new(RwLock::new(sync_manager)),
                config: Arc::new(RwLock::new(config)),
            }));

            app.manage(app_state);

            // Create tray menu
            let tray_menu = create_tray_menu(app.handle())?;

            // Build tray icon
            let _tray = TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .tooltip("ClipSync")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            // Handle window close event - minimize to tray instead
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        // Prevent close, hide instead
                        api.prevent_close();
                        let _ = window_clone.hide();
                    }
                });

                // Start minimized if configured
                if start_minimized {
                    let _ = window.hide();
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            connect_to_server,
            disconnect_from_server,
            toggle_sync,
            get_config,
            show_window,
            hide_window,
            set_autostart,
            set_start_minimized,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
