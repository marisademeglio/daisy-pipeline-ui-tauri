use tauri::{AppHandle, CustomMenuItem, Menu, Submenu, State};
use minidom::Element;

pub struct JobMenuItem {
    id: String,
    label: String
}

pub fn build_menu(history_list: Option<Vec<JobMenuItem>>) -> Menu {
    // do a custom quit instead of a native menu item 
    // because no events get emitted from the "quit" native menu item
    // https://github.com/tauri-apps/tauri/issues/3124
    let mut quit = CustomMenuItem::new("custom_quit".to_string(), "Quit");
    quit = quit.accelerator("Cmd+Q");
    
    let app_submenu = Submenu::new("", Menu::new()
            .add_item(quit));
    
    let history_submenu_contents = Menu::new();
    for item in history_list.unwrap_or(vec![]) {
        let history_menu_item = CustomMenuItem::new(item.id, item.label);
        history_submenu_contents.clone().add_item(history_menu_item);
    }

    let history_submenu = Submenu::new("History", history_submenu_contents);

    let menu = Menu::new()
        .add_submenu(app_submenu)
        .add_submenu(history_submenu);
    return menu;
}

pub async fn update_menus(jobs: String, app_handle: AppHandle) {
    println!("populate history menu");

    const NS: &'static str = "http://www.daisy.org/ns/pipeline/data";
    
    let mut history_list: Vec<JobMenuItem> = Vec::new();
    // for child in jobs.children() {
    //     let id = child.attr("id").unwrap().to_string();
    //     let job_resp = pipeline_api::get_job(id.clone()).await;
    //     let job_resp_root: Element = job_resp.parse().unwrap();
    //     let script = job_resp_root.get_child("script", NS).unwrap();
    //     let nicename = script.get_child("nicename", NS).unwrap().text();
    //     let history_menu_item = JobMenuItem { id: id, label: nicename};
    //     history_list.push(history_menu_item);        
    // }
    
    // basically build a new menu and attach it to the app
    let updated_menu = build_menu(Some(history_list));
    // TODO how to attach it
    
}