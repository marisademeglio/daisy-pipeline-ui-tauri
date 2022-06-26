use std::{thread, time};
use uuid::Uuid;
use tauri::State;
use std::sync::Mutex;
use std::sync::Arc;
use tauri::async_runtime::Mutex as AsyncMutex;

use crate::mock_store;

// pretend to start the pipeline
pub async fn start_pipeline()->Result<(), Box<dyn std::error::Error>> {
    println!("Mock start_pipeline");
    thread::sleep(time::Duration::from_millis(3000));
    Ok(())
}

pub async fn is_alive() ->  bool {
    thread::sleep(time::Duration::from_millis(1000));
    return true;
}

pub async fn halt() -> bool {
    thread::sleep(time::Duration::from_secs(1));
    return true;
}

pub async fn run_job_demo(jobs: State<'_, mock_store::Jobs>) -> bool {
    println!("Mock run_job_demo");
    
    let job = Mutex::new(mock_store::Job { 
        id: Uuid::new_v4().to_string(),  
        status: "RUNNING".to_string() 
    });

    
    let mut jobs_ = jobs.0.lock().unwrap();
    jobs_.push(job);
    
    std::mem::drop(jobs_); // release the mutex

    // let jobsarc = Arc::new(jobs.0.lock().unwrap());
    // tauri::async_runtime::spawn(async move {
    //     let jobsarc_clone = jobsarc.clone();
    //     thread::sleep(time::Duration::from_secs(2));
    //     jobsarc_clone.last_mut().unwrap().lock().unwrap().status = "SUCCESS".to_string();
    // });
    

    return true;
}

pub async fn get_jobs(jobs: State<'_, mock_store::Jobs>) -> String {
    println!("Mock get_jobs");
    let mut jobs_ = jobs.0.lock().unwrap();
    let mut iter = jobs_.iter_mut();
    
    let mut jobs_xml = String::new();
    for job in iter {
        let j = job.lock().unwrap();
        jobs_xml.push_str(&(format_job_xml(j).to_string()));
    }
    let mut xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?><jobs>{jobs_xml}</jobs>", jobs_xml = jobs_xml);
    return xml.to_string();
}

pub async fn get_job(id: String, jobs: State<'_, mock_store::Jobs>) -> String {
    println!("Mock get_job");
    let jobs_ = jobs.0.lock().unwrap();
    let mut iter = jobs_.iter();
    let job_ = iter.find(|&j| j.lock().unwrap().id == id);
    
    if let None = job_ {
        return "<?xml version=\"1.0\" encoding=\"UTF-8\"?><error></error>".to_string();
    }
    let job = job_.unwrap().lock().unwrap();
    
    let xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>{}", 
    format_job_xml(job));

    return xml.to_string();
}

pub async fn delete_job(id: String, jobs: State<'_, mock_store::Jobs>) -> bool {
    println!("Mock delete_job");
    let mut jobs_ = jobs.0.lock().unwrap();
    jobs_.retain(|j| j.lock().unwrap().id != id);
    return true;
}

fn format_job_xml(job: std::sync::MutexGuard<'_, mock_store::Job, >) -> String {
    return format!("<job xmlns=\"http://www.daisy.org/ns/pipeline/data\" 
    href=\"http://localhost:8181/ws/jobs/{id}\" 
    id=\"{id}\" priority=\"medium\" status=\"{status}\">
    <script href=\"http://localhost:8181/ws/scripts/daisy202-to-epub3\" id=\"daisy202-to-epub3\" 
    input-filesets=\"daisy202\" output-filesets=\"epub3\">
    <nicename>DAISY 2.02 to EPUB 3</nicename>
    <description>Transforms a DAISY 2.02 publication into an EPUB 3 publication.</description>
    <version>2.0.5</version>
    </script>
    <log href=\"http://localhost:8181/ws/jobs/{id}/log\"/>
    <results href=\"http://localhost:8181/ws/jobs/{id}/result\" 
    mime-type=\"application/zip\">
    <result from=\"option\" 
    href=\"http://localhost:8181/ws/jobs/{id}/result/option/output\" 
    mime-type=\"application/zip\" name=\"output\" nicename=\"EPUB 3\">
    <result file=\"file:/Path/to/project/daisy-pipeline/data/jobs/{id}/output/output/skipdemo-amsterdam.epub\" 
    href=\"http://localhost:8181/ws/jobs/{id}/result/option/output/idx/output/skipdemo-amsterdam.epub\" 
    size=\"1940333\"/>
    </result></results></job>", id=job.id, status=job.status);
}