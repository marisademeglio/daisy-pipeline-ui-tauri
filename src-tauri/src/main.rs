#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{env};
use dotenv;
use tauri::{
    Manager,
    Event, RunEvent, WindowEvent
  };
mod menus;

fn main() {
    dotenv::dotenv().ok();
    let menu = menus::build_menu(None);
    let app = tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "custom_quit" => {
                    println!("Menu quit");
                    tauri::async_runtime::spawn(async move {
                        do_exit(event.window().app_handle()).await;
                    });
                },
                "history" => {
                    // let window = event.window().clone();
                    // let _ = window.emit("goto-tab", "start".to_string());
                },
                "view_jobs_window" => {
                    if let jobs_window = event.window().app_handle().get_window("jobs").unwrap() {
                        if jobs_window.is_visible().unwrap() {
                            jobs_window.hide();
                        }
                        else {
                            jobs_window.show();
                        }
                    }
                    else {
                        println!("Jobs window error");
                    }
                    
                },
                _ => {println!("Menu {}", event.menu_item_id())}
            }
        })
        .invoke_handler(tauri::generate_handler![
            update_menu
        ])    
        .build(tauri::generate_context!())
        .expect("error while building tauri application");
    
    app.run(|app_handle, event| match event {
        // Application is ready (triggered only once)
        RunEvent::Ready => {
            println!("RunEvent::Ready");
        }

        // Triggered when a window is trying to close
        RunEvent::WindowEvent {
            label,
            event: WindowEvent::CloseRequested { api, .. },
            ..
        } => {
            if label == "main" {
                println!("WindowEvent::CloseRequested");
                let app_handle = app_handle.clone();
                let _window = app_handle.get_window(&label).unwrap();
                tauri::async_runtime::spawn(async move {
                    do_exit(app_handle).await;
                });
            }
            else if label == "jobs" {
                let app_handle = app_handle.clone();
                let window = app_handle.get_window(&label).unwrap();
                window.hide().unwrap();
                api.prevent_close();
            }
        }

        _ => {}
    });
}


async fn do_exit(app_handle: tauri::AppHandle) {
    println!("Do exit");
    app_handle.exit(0);
}

// reminder: this issue comes up with async [command]s https://github.com/tauri-apps/tauri/issues/2533
#[tauri::command]
async fn update_menu(app_handle: tauri::AppHandle, jobs: String) -> Result<bool, String> {
    menus::update_menus(jobs, app_handle);
    Ok(true)
}

