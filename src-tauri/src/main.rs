#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod attack;
mod phone;
mod services;

use crate::attack::send;
use crate::phone::{Country, FormatterErrors, Phone};
use crate::services::Victim;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            format_phone_ru,
            show_dialog_error,
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
async fn show_dialog_error(app: tauri::AppHandle, e: String) {
    let message = match e.as_str() {
        "0" => app.dialog().message("Неправильная длина номера"),
        "1" => app
            .dialog()
            .message("Неправильный шаблон номера\nОн должен быть похожим на 7 (9xx) xxx-xx-xx"),
        "2" => app.dialog().message("Подождите..."),
        _ => app.dialog().message(""),
    };

    if e == "0" || e == "1" {
        message
            .kind(MessageDialogKind::Error)
            .title("Неправильно набран номер")
            .blocking_show();
    } else if e == "2" {
        message
            .kind(MessageDialogKind::Warning)
            .title("Уже атакует")
            .blocking_show();
    } else {
        message
            .kind(MessageDialogKind::Error)
            .title("Неизвестная ошибка")
            .blocking_show();
    }
}

#[tauri::command]
async fn attack(phone: String, cycles: u64) {
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
    let _ = send(victim, cycles).await;
}

#[tauri::command]
async fn show_about_window(app: tauri::AppHandle) {
    tauri::WebviewWindowBuilder::new(&app, "about", tauri::WebviewUrl::App("/about".into()))
        .title("О программе")
        .inner_size(350.0, 600.0)
        .resizable(false)
        .build()
        .unwrap();
}
