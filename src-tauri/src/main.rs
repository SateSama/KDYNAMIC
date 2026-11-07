// Pas de console noire sur Windows en release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{LogicalSize, Manager, PhysicalPosition, WebviewWindow};

/// Centre la fenêtre en haut de l'écran où elle se trouve.
fn center_top(win: &WebviewWindow) -> Result<(), String> {
    let monitor = win
        .current_monitor()
        .map_err(|e| e.to_string())?
        .ok_or("aucun écran détecté")?;
    let msize = monitor.size();
    let mpos = monitor.position();
    let wsize = win.outer_size().map_err(|e| e.to_string())?;
    let x = mpos.x + (msize.width as i32 - wsize.width as i32) / 2;
    win.set_position(PhysicalPosition::new(x, mpos.y))
        .map_err(|e| e.to_string())
}

/// Appelée par l'interface à chaque changement d'état de l'île.
#[tauri::command]
fn resize_island(app: tauri::AppHandle, width: f64, height: f64) -> Result<(), String> {
    let win = app.get_webview_window("main").ok_or("fenêtre introuvable")?;
    win.set_size(LogicalSize::new(width, height))
        .map_err(|e| e.to_string())?;
    center_top(&win)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![resize_island])
        .setup(|app| {
            if let Some(win) = app.get_webview_window("main") {
                let _ = center_top(&win);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("erreur au lancement de PC Dynamic Island");
}
