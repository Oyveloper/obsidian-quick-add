mod commands;

use commands::obsidian::{add_task_to_daily_note, get_vaults};
use tauri::{AppHandle, Manager, RunEvent, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[tauri::command]
fn hide_window(app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.destroy();
    }
}

#[tauri::command]
fn show_window(app: AppHandle) {
    create_window(&app);
}

fn create_window(app: &AppHandle) {
    // Check if window already exists
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_focus();
        return;
    }

    // Create a new window
    let _ = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("Obsidian Quick Add")
        .inner_size(650.0, 72.0)
        .resizable(false)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .center()
        .visible(true)
        .skip_taskbar(true)
        .shadow(false)
        .focused(true)
        .build();
}

fn toggle_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        // Window exists, destroy it
        let _ = window.destroy();
    } else {
        // Window doesn't exist, create it
        create_window(app);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        toggle_window(app);
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            get_vaults,
            add_task_to_daily_note,
            hide_window,
            show_window
        ])
        .setup(|app| {
            // Register global shortcut Cmd+Shift+K
            let shortcut = Shortcut::new(
                Some(Modifiers::SUPER | Modifiers::SHIFT),
                Code::KeyK,
            );

            app.global_shortcut().register(shortcut)?;

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, event| {
            // Prevent app from exiting when all windows are closed
            if let RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
}
