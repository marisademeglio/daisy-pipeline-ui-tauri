#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;
use dotenv;
use tauri::{CustomMenuItem, Menu, Submenu};
use tauri::{
    Manager,
    RunEvent, WindowEvent
  };
mod pipeline_api;

#[tauri::command]
async fn start_pipeline() -> String {
    let res = pipeline_api::start_pipeline().await;
    match res {
        Ok(()) => {
            println!("Pipeline service started");
            return "Pipeline WS started".to_string();
        }
        Err(e) => {
            println!("{e}");
            return "Error".to_string();
        }
    };
}
#[tauri::command]
async fn is_pipeline_alive() -> bool {
    return pipeline_api::is_alive().await;
}
#[tauri::command]
async fn halt_pipeline() {
    pipeline_api::halt().await;
}
#[tauri::command]
async fn run_predetermined_job() -> bool {
   return pipeline_api::run_job_demo().await;
}
#[tauri::command]
async fn get_jobs() -> String {
    let resp = pipeline_api::get_jobs().await;
    // println!("{}", resp);
    return resp;
}
#[tauri::command]
async fn get_job(id: String) -> String {
    let resp = pipeline_api::get_job(id).await;
    return resp;
}

#[tauri::command]
async fn delete_job(id: String) -> bool {
    let resp = pipeline_api::delete_job(id).await;
    return resp;
}

fn main() {
    dotenv::dotenv().ok();
    let menu = build_menu();
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_pipeline, 
            is_pipeline_alive, 
            halt_pipeline, 
            run_predetermined_job,
            get_jobs, 
            get_job,
            delete_job])
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "custom_quit" => {
                    println!("Menu quit");
                    tauri::async_runtime::spawn(async move {
                        do_exit().await;
                    });
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
            // start the pipeline
            tauri::async_runtime::spawn(async move {
                let is_alive = pipeline_api::is_alive().await;
                if is_alive {
                    println!("Pipeline is already running");
                }
                else {
                    let _res = pipeline_api::start_pipeline().await;
                }
            });
            let app_handle = app_handle.clone();
            let window = app_handle.get_window("main").unwrap();
            window.set_title("DAISY Pipeline").unwrap();
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
                do_exit().await;
            });
            
        }

        _ => {}
    });
}

fn build_menu() -> Menu {
    // do a custom quit instead of a native menu item 
    // because no events get emitted from the "quit" native menu item
    // https://github.com/tauri-apps/tauri/issues/3124
    let mut quit = CustomMenuItem::new("custom_quit".to_string(), "Quit");
    quit = quit.accelerator("Cmd+Q");
    let submenu = Submenu::new("File", Menu::new()
            // .add_item(CustomMenuItem::new("about".to_string(), "About"))
            .add_item(quit));
    
        
    let menu = Menu::new()
        .add_submenu(submenu);
    return menu;
}

async fn do_exit() { //app_handle: tauri::AppHandle) {
    println!("Do exit");
    // this is where we would warn people if they try to quit and there are still jobs running
    pipeline_api::halt().await;
    
    // it would be nice to call this in case it does anything valuable behind the scenes
    // but how to get the app_handle from menu event trigger (one of the source events for exiting is the Quit menu item)
    // app_handle.exit(0);
    std::process::exit(0);
}