#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{env, thread, time};
use dotenv;
use tauri::{
    Manager,
    RunEvent, WindowEvent
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
            event: WindowEvent::CloseRequested { api: _, .. },
            ..
        } => {
            println!("WindowEvent::CloseRequested");
            let app_handle = app_handle.clone();
            let _window = app_handle.get_window(&label).unwrap();
            tauri::async_runtime::spawn(async move {
                do_exit(app_handle).await;
            });
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
async fn update_menu(app_handle: tauri::AppHandle) -> Result<bool, String> {
    
    Ok(true)
}

// // reminder: this issue comes up with async [command]s https://github.com/tauri-apps/tauri/issues/2533
// #[tauri::command]
// async fn is_pipeline_alive() -> Result<bool, String> {
//     let is_alive = pipeline_api::is_alive().await;
//     Ok(is_alive)
// }
// #[tauri::command]
// async fn run_predetermined_job() -> Result<bool, String> {
//     let success = pipeline_api::run_job_demo().await;
//     Ok(success)
// }

// #[tauri::command]
// async fn get_jobs(app_handle: tauri::AppHandle) -> Result<String, String> {
//     let resp = pipeline_api::get_jobs().await;
//     menus::update_menus(resp.clone(), app_handle).await;
//     Ok(resp)
// }

// #[tauri::command]
// async fn get_job(id: String) -> Result<String, String> {
//     let resp = pipeline_api::get_job(id).await;
//     Ok(resp)
// }

// #[tauri::command]
// async fn delete_job(id: String) -> Result<bool, String> {
//     let resp = pipeline_api::delete_job(id).await;
//     Ok(resp)
// }