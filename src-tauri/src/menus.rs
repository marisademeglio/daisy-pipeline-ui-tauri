use tauri::{AppHandle, CustomMenuItem, Menu, Submenu, Manager};
use minidom::Element;

#[path = "pipeline_api.rs"] mod pipeline_api;

pub fn build_menu() -> Menu {
    // do a custom quit instead of a native menu item 
    // because no events get emitted from the "quit" native menu item
    // https://github.com/tauri-apps/tauri/issues/3124
    let mut quit = CustomMenuItem::new("custom_quit".to_string(), "Quit");
    quit = quit.accelerator("Cmd+Q");
    
    let app_submenu = Submenu::new("", Menu::new()
            .add_item(quit));
    let history_submenu = Submenu::new("History", Menu::new());
        
    let menu = Menu::new()
        .add_submenu(app_submenu)
        .add_submenu(history_submenu);
    return menu;
}

// jobs_xml is an XML string response from the pipeline endpoint /jobs
pub async fn populate_history_menu(jobs_xml: String, app_handle: AppHandle) {
    println!("populate history menu");

    const NS: &'static str = "http://www.daisy.org/ns/pipeline/data";
    let root: Element = match jobs_xml.parse::<Element>() {
        Ok(root) => {
            root
        },
        Err(e) => {
            // this error will happen a few times at the start, 
            // before the pipeline service has been started
            return;
        }
    };
    for child in root.children() {
        let id = child.attr("id").unwrap().to_string();
        let job_resp = pipeline_api::get_job(id).await;
        let job_resp_root: Element = job_resp.parse().unwrap();
        let script = job_resp_root.get_child("script", NS).unwrap();
        let nicename = script.get_child("nicename", NS).unwrap().text();

        // how to get a submenu ?
        //let history_menu = app_handle.get_window("main").unwrap().menu_handle();
        
    }
    // TODO how to get the app or main_window from here?
}