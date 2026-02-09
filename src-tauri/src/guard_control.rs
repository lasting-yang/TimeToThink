use tauri::{ActivationPolicy, AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

const KIOSK_MODE_ENV: &str = "TTT_KIOSK_MODE";

fn kiosk_mode_enabled() -> bool {
    let raw = std::env::var(KIOSK_MODE_ENV).unwrap_or_else(|_| "1".to_string());
    !matches!(raw.trim().to_ascii_lowercase().as_str(), "0" | "false" | "no" | "off")
}

#[cfg(target_os = "macos")]
fn apply_macos_presentation_options(app: &AppHandle, enabled: bool) {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSApplicationPresentationOptions};

    let _ = app.run_on_main_thread(move || {
        let Some(mtm) = MainThreadMarker::new() else {
            return;
        };
        let ns_app = NSApplication::sharedApplication(mtm);
        let options = if enabled {
            NSApplicationPresentationOptions::HideDock
                | NSApplicationPresentationOptions::HideMenuBar
                | NSApplicationPresentationOptions::DisableAppleMenu
                | NSApplicationPresentationOptions::DisableProcessSwitching
                | NSApplicationPresentationOptions::DisableForceQuit
                | NSApplicationPresentationOptions::DisableSessionTermination
                | NSApplicationPresentationOptions::DisableHideApplication
        } else {
            NSApplicationPresentationOptions::Default
        };
        ns_app.setPresentationOptions(options);
    });
}

fn stretch_guard_to_monitor(app: &AppHandle, guard_window: &WebviewWindow) -> Result<(), String> {
    let monitor = guard_window
        .current_monitor()
        .map_err(|e| e.to_string())?
        .or_else(|| app.primary_monitor().ok().flatten());

    if let Some(monitor) = monitor {
        guard_window
            .set_position(*monitor.position())
            .map_err(|e| e.to_string())?;
        guard_window
            .set_size(*monitor.size())
            .map_err(|e| e.to_string())?;
    }

    guard_window.set_decorations(false).map_err(|e| e.to_string())?;
    guard_window.set_resizable(false).map_err(|e| e.to_string())?;
    guard_window.set_closable(false).map_err(|e| e.to_string())?;
    guard_window.set_minimizable(false).map_err(|e| e.to_string())?;
    guard_window.set_maximizable(false).map_err(|e| e.to_string())?;
    guard_window.set_skip_taskbar(true).map_err(|e| e.to_string())?;
    guard_window.set_always_on_top(true).map_err(|e| e.to_string())?;

    #[cfg(target_os = "macos")]
    {
        // Important: simple fullscreen keeps macOS menu bar behavior; disable it.
        let _ = guard_window.set_simple_fullscreen(false);
        guard_window
            .set_visible_on_all_workspaces(true)
            .map_err(|e| e.to_string())?;
    }

    if let Err(e) = guard_window.set_fullscreen(true) {
        eprintln!("native fullscreen failed: {}", e);
    }

    Ok(())
}

pub async fn show_guard(app: &AppHandle) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let _ = app.set_activation_policy(ActivationPolicy::Regular);
        apply_macos_presentation_options(app, kiosk_mode_enabled());
    }

    if let Some(main_window) = app.get_webview_window("main") {
        let _ = main_window.hide();
    }

    // Try to get existing guard window
    if let Some(guard_window) = app.get_webview_window("breakguard") {
        guard_window.show().map_err(|e| e.to_string())?;
        stretch_guard_to_monitor(app, &guard_window)?;
        guard_window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    // Create new guard window if it doesn't exist
    let guard = WebviewWindowBuilder::new(
        app,
        "breakguard",
        WebviewUrl::App("index.html".into())
    )
    .title("Break Time")
    .always_on_top(true)
    .resizable(false)
    .decorations(false)
    .build()
    .map_err(|e| e.to_string())?;
    stretch_guard_to_monitor(app, &guard)?;

    Ok(())
}

pub async fn hide_guard(app: &AppHandle) -> Result<(), String> {
    if let Some(guard_window) = app.get_webview_window("breakguard") {
        let _ = guard_window.set_fullscreen(false);
        guard_window.close().map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        apply_macos_presentation_options(app, false);
        let _ = app.set_activation_policy(ActivationPolicy::Accessory);
    }

    Ok(())
}

pub async fn ensure_guard_focused(app: &AppHandle) -> Result<(), String> {
    if let Some(guard_window) = app.get_webview_window("breakguard") {
        if !guard_window.is_visible().map_err(|e| e.to_string())? {
            guard_window.show().map_err(|e| e.to_string())?;
        }
        stretch_guard_to_monitor(app, &guard_window)?;
        guard_window.set_focus().map_err(|e| e.to_string())?;
        guard_window.set_always_on_top(true).map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub async fn start_guard_polling(app: AppHandle, engine: crate::timer_engine::SharedTimerEngine) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(500));
    loop {
        interval.tick().await;

        let engine_guard = engine.lock().await;
        let is_break = engine_guard.get_state().is_break() && engine_guard.is_running();
        drop(engine_guard);

        if is_break {
            if let Err(e) = ensure_guard_focused(&app).await {
                eprintln!("Guard polling error: {}", e);
            }
        } else if let Some(guard_window) = app.get_webview_window("breakguard") {
            match guard_window.is_visible() {
                Ok(true) => {
                    if let Err(e) = hide_guard(&app).await {
                        eprintln!("Guard hide polling error: {}", e);
                    }
                }
                Ok(false) => {}
                Err(e) => eprintln!("Guard visibility polling error: {}", e),
            }
        }
    }
}
