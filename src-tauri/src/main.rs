#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod attack;
mod phone;
mod services;

use crate::phone::{Country, FormatterErrors, Phone};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![echo, format_phone_ru])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn echo(msg: &str) {
    println!("Echo {}", msg);
}

#[tauri::command]
fn format_phone_ru(numbers: &str) -> String {
    match Phone::new(numbers.to_string(), Country::Ru) {
        Ok(t) => t.phone,
        Err(e) => match e {
            FormatterErrors::IncorrectPatter => {
                println!("Incorrect pattern\nNumber must be like 7 (9xx) xxx-xx-xx\n");
                "0".to_string()
            }
            FormatterErrors::IncorrectLength => {
                println!("Incorrect number length\n");
                "1".to_string()
            }
        },
    }
}
