#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;
use dotenv;
use tauri::{
    Manager,
    RunEvent, WindowEvent
  };


mod commands;
mod pipeline_api;
mod menus;

fn main() {
    dotenv::dotenv().ok();
    let menu = menus::build_menu();
    let app = tauri::Builder::default()
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
               // start the pipeline
                let is_alive = pipeline_api::is_alive().await;
                if is_alive {
                    println!("Pipeline is already running");
                }
                else {
                    let _res = pipeline_api::start_pipeline().await;
                }
        
                // After it's done, close the splashscreen and display the main window
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::is_pipeline_alive, 
            commands::run_predetermined_job,
            commands::get_jobs, 
            commands::get_job,
            commands::delete_job])
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
    // this is where we would warn people if they try to quit and there are still jobs running
    pipeline_api::halt().await;
    app_handle.exit(0);
}
