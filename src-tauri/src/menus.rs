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
        // .add_submenu(history_submenu)
        ;
    return menu;
}

pub fn update_menus(jobs: String, app_handle: AppHandle) {
    println!("populate history menu");

    
    let mut history_list: Vec<JobMenuItem> = Vec::new();
    let json: Vec<serde_json::Value> =
        serde_json::from_str(jobs.as_str()).unwrap();
    // println!("{}", json);
    for item in json {
        let id = &item["id"];
        let script = &item["scriptName"];
        let history_menu_item = JobMenuItem{id: id.to_string(), label: script.to_string()};
        history_list.push(history_menu_item);
    }
    
    // basically build a new menu and attach it to the app
    let updated_menu = build_menu(Some(history_list));

    // TODO attach it to the window
    // let main_window = app_handle.get_window("main").unwrap();
    //   let menu_handle = main_window.menu_handle();
    
}