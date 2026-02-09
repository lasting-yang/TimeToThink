#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod guard_control;
mod macos_lock;
mod timer_engine;
mod types;

use guard_control::{hide_guard, show_guard, start_guard_polling};
use macos_lock::lock_screen;
use std::sync::Arc;
use tauri::image::Image;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{ActivationPolicy, AppHandle, Listener, Manager, RunEvent, State};
use types::TimerUpdate;
use timer_engine::{start_timer_loop, SharedTimerEngine, TimerEngine};
use tokio::sync::Mutex;

const FALLBACK_TRAY_ICON: [u8; 4] = [255, 255, 255, 255];
const KIOSK_MODE_ENV: &str = "TTT_KIOSK_MODE";

fn kiosk_mode_enabled() -> bool {
    let raw = std::env::var(KIOSK_MODE_ENV).unwrap_or_else(|_| "1".to_string());
    !matches!(raw.trim().to_ascii_lowercase().as_str(), "0" | "false" | "no" | "off")
}

fn toggle_main_window(app: &AppHandle) {
    if let Some(main_window) = app.get_webview_window("main") {
        let is_visible = main_window.is_visible().unwrap_or(false);
        let is_focused = main_window.is_focused().unwrap_or(false);

        if is_visible && is_focused {
            let _ = main_window.hide();
        } else {
            let _ = main_window.show();
            let _ = main_window.set_focus();
        }
    }
}

#[tauri::command]
async fn start_timer(engine: State<'_, SharedTimerEngine>, app: AppHandle) -> Result<(), String> {
    let mut engine_guard = engine.lock().await;
    engine_guard.start(&app).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn pause_timer(engine: State<'_, SharedTimerEngine>, app: AppHandle) -> Result<(), String> {
    let mut engine_guard = engine.lock().await;
    engine_guard.pause(&app).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn reset_timer(engine: State<'_, SharedTimerEngine>, app: AppHandle) -> Result<(), String> {
    let mut engine_guard = engine.lock().await;
    engine_guard.reset(&app).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn skip_break(
    engine: State<'_, SharedTimerEngine>,
    app: AppHandle,
) -> Result<(), String> {
    {
        let mut engine_guard = engine.lock().await;
        engine_guard.skip_break();
        engine_guard.emit_update(&app).map_err(|e| e.to_string())?;
    }
    hide_guard(&app).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_timer_state(engine: State<'_, SharedTimerEngine>) -> Result<TimerUpdate, String> {
    let engine_guard = engine.lock().await;
    Ok(engine_guard.get_update())
}

fn main() {
    let app = tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            // Initialize timer engine
            let engine: SharedTimerEngine = Arc::new(Mutex::new(TimerEngine::new()));

            // Start as a menubar-style app: keep main window hidden initially.
            if let Some(main_window) = app.get_webview_window("main") {
                let _ = main_window.hide();
            }

            if let Some(guard_window) = app.get_webview_window("breakguard") {
                let _ = guard_window.hide();
            }

            let app_handle_for_tray = app.handle().clone();
            let mut tray_builder = TrayIconBuilder::new()
                .tooltip("TimeToThink")
                .on_tray_icon_event(move |_tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        toggle_main_window(&app_handle_for_tray);
                    }
                });

            if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            } else {
                tray_builder = tray_builder.icon(Image::new(&FALLBACK_TRAY_ICON, 1, 1));
            }

            tray_builder.build(app)?;

            // Store engine in app state
            app.manage(engine.clone());

            // Listen for lock_screen event from timer engine
            app.listen("lock_screen", move |_| {
                if let Err(e) = lock_screen() {
                    eprintln!("Lock screen error: {}", e);
                }
            });

            // Listen for show_guard event from timer engine
            let app_handle_for_guard = app.handle().clone();
            app.listen("show_guard", move |_| {
                let handle = app_handle_for_guard.clone();
                tauri::async_runtime::spawn(async move {
                    let _ = show_guard(&handle).await;
                });
            });

            // Listen for hide_guard event from timer engine
            let app_handle_for_hide = app.handle().clone();
            app.listen("hide_guard", move |_| {
                let handle = app_handle_for_hide.clone();
                tauri::async_runtime::spawn(async move {
                    let _ = hide_guard(&handle).await;
                });
            });

            // Start timer loop
            let app_handle_for_timer = app.handle().clone();
            tauri::async_runtime::spawn(start_timer_loop(app_handle_for_timer, engine.clone()));

            // Start guard polling
            let app_handle_for_polling = app.handle().clone();
            tauri::async_runtime::spawn(start_guard_polling(app_handle_for_polling, engine.clone()));

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            start_timer,
            pause_timer,
            reset_timer,
            skip_break,
            get_timer_state
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle, event| {
        if !kiosk_mode_enabled() {
            return;
        }

        if let RunEvent::ExitRequested { api, .. } = event {
            if let Some(engine) = app_handle.try_state::<SharedTimerEngine>() {
                let should_block_exit = match engine.try_lock() {
                    Ok(engine_guard) => engine_guard.get_state().is_break() && engine_guard.is_running(),
                    Err(_) => true,
                };

                if should_block_exit {
                    eprintln!("Exit blocked: kiosk mode is active during break");
                    api.prevent_exit();
                }
            }
        }
    });
}
