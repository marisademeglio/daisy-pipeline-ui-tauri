#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;
use dotenv;
use tauri::{
    Manager,
    RunEvent, WindowEvent, State
  };

mod menus;
mod pipeline_api;
mod real_pipeline_api;
mod mock_store;
mod mock_pipeline_api;
mod error;

fn main() {
    dotenv::dotenv().ok();
    let menu = menus::build_menu(None);
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
        .manage(mock_store::Jobs(Default::default()))
        .invoke_handler(tauri::generate_handler![
            is_pipeline_alive, 
            run_predetermined_job,
            get_jobs, 
            get_job,
            delete_job
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
    // this is where we would warn people if they try to quit and there are still jobs running
    pipeline_api::halt().await;
    app_handle.exit(0);
}

// reminder: this issue comes up with async [command]s https://github.com/tauri-apps/tauri/issues/2533
#[tauri::command]
async fn is_pipeline_alive() -> Result<bool, String> {
    let is_alive = pipeline_api::is_alive().await;
    Ok(is_alive)
}
#[tauri::command]
async fn run_predetermined_job(jobs: State<'_, mock_store::Jobs>) -> Result<bool, String> {
    let success = pipeline_api::run_job_demo(jobs).await;
    Ok(success)
}

#[tauri::command]
async fn get_jobs(app_handle: tauri::AppHandle, jobs: State<'_, mock_store::Jobs>) -> Result<String, String> {
    let resp = pipeline_api::get_jobs(jobs.clone()).await;
    menus::update_menus(resp.clone(), app_handle, jobs).await;
    Ok(resp)
}

#[tauri::command]
async fn get_job(id: String, jobs: State<'_, mock_store::Jobs>) -> Result<String, String> {
    let resp = pipeline_api::get_job(id, jobs).await;
    Ok(resp)
}

#[tauri::command]
async fn delete_job(id: String, jobs: State<'_, mock_store::Jobs>) -> Result<bool, String> {
    let resp = pipeline_api::delete_job(id, jobs).await;
    Ok(resp)
}