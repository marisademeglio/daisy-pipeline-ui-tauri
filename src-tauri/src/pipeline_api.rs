use std::env;
use tauri::State;
use crate::mock_store;
use crate::real_pipeline_api;
use crate::mock_pipeline_api;



// start the pipeline
pub async fn start_pipeline()->Result<(), Box<dyn std::error::Error>> {
    let mode = env::var("MOCK_PIPELINE")?;
    println!("mode {}", mode);
    if mode == "false" {
        return real_pipeline_api::start_pipeline().await;
    }
    else {
        return mock_pipeline_api::start_pipeline().await;
    }
}

pub async fn is_alive() ->  bool {
    let mode = env::var("MOCK_PIPELINE");
    match mode {
        Ok(val) => {
            if val == "false" {
                return real_pipeline_api::is_alive().await;
            }
            else {
                return mock_pipeline_api::is_alive().await;
            }
        }
        Err(_e) => {
            return real_pipeline_api::is_alive().await;
        }
    }
}

pub async fn halt() -> bool {
    let mode = env::var("MOCK_PIPELINE");
    match mode {
        Ok(val) => {
            if val == "false" {
                return real_pipeline_api::halt().await;
            }
            else {
                return mock_pipeline_api::halt().await;
            }
        }
        Err(_e) => {
            return real_pipeline_api::halt().await;
        }
    }
}

pub async fn run_job_demo(jobs: State<'_, mock_store::Jobs>) -> bool {
    let mode = env::var("MOCK_PIPELINE");
    match mode {
        Ok(val) => {
            if val == "false" {
                return real_pipeline_api::run_job_demo().await;
            }
            else {
                return mock_pipeline_api::run_job_demo(jobs).await;
            }
        }
        Err(_e) => {
            return real_pipeline_api::run_job_demo().await;
        }
    }
}

pub async fn get_jobs(jobs: State<'_, mock_store::Jobs>) -> String {
    let mode = env::var("MOCK_PIPELINE");
    match mode {
        Ok(val) => {
            if val == "false" {
                return real_pipeline_api::get_jobs().await;
            }
            else {
                return mock_pipeline_api::get_jobs(jobs).await;
            }
        }
        Err(_e) => {
            return real_pipeline_api::get_jobs().await;
        }
    }
}

pub async fn get_job(id: String, jobs: State<'_, mock_store::Jobs>) -> String {
    let mode = env::var("MOCK_PIPELINE");
    match mode {
        Ok(val) => {
            if val == "false" {
                return real_pipeline_api::get_job(id).await;
            }
            else {
                return mock_pipeline_api::get_job(id, jobs).await;
            }
        }
        Err(_e) => {
            return real_pipeline_api::get_job(id).await;
        }
    }
}

pub async fn delete_job(id: String, jobs: State<'_, mock_store::Jobs>) -> bool {
    let mode = env::var("MOCK_PIPELINE");
    match mode {
        Ok(val) => {
            if val == "false" {
                return real_pipeline_api::delete_job(id).await;
            }
            else {
                return mock_pipeline_api::delete_job(id, jobs).await;
            }
        }
        Err(_e) => {
            return real_pipeline_api::delete_job(id).await;
        }
    }
}