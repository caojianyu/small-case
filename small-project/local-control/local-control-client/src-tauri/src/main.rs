#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

use core::time;
use std::thread;

use rusqlite::{params, Connection};
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

lazy_static! {
    static ref CONN: Mutex<Connection> =
        Mutex::new(Connection::open("D:\\tauri\\local_control.db").unwrap());
}

#[tauri::command]
fn order_record(name: String, path: String) {
    let conn = CONN.lock().unwrap();

    // conn.execute(
    //     "CREATE TABLE order_record (
    //         id    INTEGER PRIMARY KEY,
    //         name  TEXT NOT NULL,
    //         path  TEXT NOT NULL
    //     )",
    //     (), // empty list of parameters.
    // )
    // .unwrap();

    conn.execute(
        "INSERT INTO order_record (name, path) VALUES (?1, ?2)",
        params![name, path],
    )
    .unwrap();
}

#[tauri::command]
fn get_order_num() -> usize {
    let conn = CONN.lock().unwrap();

    let stmt = conn.prepare("SELECT COUNT(id) FROM order_record");

    let mut count = 0;

    match stmt {
        Ok(mut v) => {
            let mut rows = v.query([]).unwrap();
            while let Some(row) = rows.next().unwrap() {
                count = row.get(0).unwrap();
            }
        }
        Err(e) => {
            println!("{:?}", e)
        }
    }

    count
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let tray_menu = SystemTrayMenu::new().add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            thread::spawn(move || loop {
                let resp = reqwest::blocking::get("http://localhost:8000/get_task_path");
                match resp {
                    Ok(v) => {
                        let text = v.text().unwrap();
                        if !text.is_empty() {
                            main_window.emit("run-task", text).unwrap();
                        }
                    }
                    Err(e) => println!("error parsing header: {:?}", e),
                }

                thread::sleep(time::Duration::from_millis(500));
            });

            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![order_record, get_order_num])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
