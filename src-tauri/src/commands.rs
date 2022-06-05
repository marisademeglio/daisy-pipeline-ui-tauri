// these commands are made available to the JS frontend by main.rs

#[path = "pipeline_api.rs"] mod pipeline_api;
#[path = "menus.rs"] mod menus;

#[tauri::command]
pub async fn is_pipeline_alive() -> bool {
    return pipeline_api::is_alive().await;
}
#[tauri::command]
pub async fn run_predetermined_job() -> bool {
   return pipeline_api::run_job_demo().await;
}

#[tauri::command]
pub async fn get_jobs(app_handle: tauri::AppHandle) -> String {
    let resp = pipeline_api::get_jobs().await;
    menus::populate_history_menu(resp.clone(), app_handle).await;
    return resp;
}
#[tauri::command]
pub async fn get_job(id: String) -> String {
    let resp = pipeline_api::get_job(id).await;
    return resp;
}

#[tauri::command]
pub async fn delete_job(id: String) -> bool {
    let resp = pipeline_api::delete_job(id).await;
    return resp;
}