// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Window};
use tauri::PhysicalPosition;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn set_window_position_right(window: Window) -> Result<(), String> {
    if let Some(monitor) = window.primary_monitor().unwrap() {
        let screen_width = monitor.size().width;
        let window_size = window.outer_size().unwrap();

        let x = screen_width - window_size.width - 16;
        let y = 16;

        window.set_position(PhysicalPosition::new(x as f64, y as f64)).unwrap();
        Ok(())
    } else {
        Err("Failed to retrieve monitor information".into())
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_window_position_right])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
