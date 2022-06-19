#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

struct Database {
    counter: Mutex<usize>,
}

#[derive(serde::Serialize)]
struct CustomResponse {
    message: String,
    value: usize,
}

#[tauri::command]
async fn increse(
    window: tauri::Window,
    number: usize,
    database: tauri::State<'_, Database>,
) -> Result<CustomResponse, String> {
    println!(
        "Called from {}, current counter value is: {}",
        window.label(),
        database.counter.lock().unwrap()
    );
    let mut counter = database.counter.lock().unwrap();
    *counter += number;
    window.set_title(&format!("Counter: {}", counter)).unwrap();
    Ok(CustomResponse {
        message: "ok".to_owned(),
        value: *counter,
    })
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .manage(Database {
            counter: Mutex::new(0),
        })
        .invoke_handler(tauri::generate_handler![increse])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}
