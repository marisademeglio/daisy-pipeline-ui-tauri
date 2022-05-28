use std::env;
use std::process::{Command, Stdio};
use std::fs::File;
use std::io::Read;
use std::fmt;
use std::{thread, time};

use reqwest;

#[derive(Debug)]
pub enum PipelineService {
    WontStart,
    TimedOut
}

impl std::error::Error for PipelineService {
    fn description(&self) -> &str {
        match *self {
            PipelineService::WontStart => "Process could not be started",
            PipelineService::TimedOut => "Timed out before process could finish starting",
        }
    }
}
impl fmt::Display for PipelineService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pipeline Service unavailable")
    }
}

// start the pipeline
pub async fn start_pipeline()->Result<(), Box<dyn std::error::Error>> {
    // use the pipeline2 executable specified by the DAISY_PIPELINE environment variable
    let pipeline_exe = env::var("DAISY_PIPELINE")?;
    let mut command = Command::new(pipeline_exe);
    match command.stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
        Ok(child) => {
            println!("Pipeline Service PID is {}", child.id());
        }, 
        Err(_e) => { return Err(PipelineService::WontStart.into()) }
    }
    
    // wait for 20 seconds max
    let mut alive = is_alive().await;
    let timeout = 20000;
    let mut total_time_spent_waiting = 0;
    while !alive && total_time_spent_waiting < timeout{
        println!("waiting for Pipeline Service to start...");
        let sleeptime = 500;
        thread::sleep(time::Duration::from_millis(sleeptime));
        total_time_spent_waiting += sleeptime;
        alive = is_alive().await;
    }

    if !alive {
        println!("Pipeline Service could not start; timeout exceeded.");
        return Err(PipelineService::TimedOut.into())
    }
    else {
        Ok(())
    }
}

pub async fn is_alive() ->  bool {
    let endpoint = "http://localhost:8181/ws/alive";
    let client = reqwest::Client::new();
    
    match client.get(endpoint).send().await {
        Ok(response) => {
            match response.status() {
                reqwest::StatusCode::OK => {
                    return true;
                },
                
                _ => { 
                    return false;
                 }
            }
        },
        Err(_e) => {
            return false;
        }
    }
}

pub async fn halt() -> bool {
    println!("Halting Pipeline Service");
    if let Ok(shutdown_key) = read_shutdown_key() {
        let endpoint = format!("http://localhost:8181/ws/admin/halt/{shutdown_key}");
        println!("{endpoint}");
        let client = reqwest::Client::new();
        
        match client.get(endpoint).send().await {
            Ok(_response) => {
                return true;
            },
            Err(_e) => {
                return false;
            }
        };
    }
    else {
        println!("Shutdown key error");
        return false;
    }
}

pub async fn run_job_demo() -> bool {
    println!("Run job demo");
    let endpoint = "http://localhost:8181/ws/jobs";
    let mut input_file = env::current_dir().unwrap();
    input_file.pop();
    input_file.push("Mountains_skip");
    input_file.push("ncc.html");
    

    let input_file = format!("file://{}", input_file.display().to_string());
    println!("{}", input_file);
    
    let job_request_xml =format!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>
    <jobRequest xmlns=\"http://www.daisy.org/ns/pipeline/data\">
        <priority>medium</priority>
        <script href=\"http://localhost:8181/ws/scripts/daisy202-to-epub3\"/>
        <option name=\"href\">{}</option>
    </jobRequest>", input_file);

    let client = reqwest::Client::new();
    println!("Submitting job");
    println!("{}", job_request_xml);
    
    match client
        .post(endpoint)
        .body(job_request_xml)
        .send().await {
            Ok(response) => {
                if response.status().is_success() {
                    return true;
                }
                else {
                    return false;
                }
            }
            Err(_e) => {
                return false;
            }
        }
}

pub async fn get_jobs() -> String {
    let endpoint = "http://localhost:8181/ws/jobs";
    let client = reqwest::Client::new();
    match client.get(endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                return response.text().await.unwrap();
            }
            else {
                return "".to_string();
            }
        }
        Err(_e) => {
            return "".to_string();
        }
    }
}

pub async fn get_job(id: String) -> String {
    let endpoint = format!("http://localhost:8181/ws/jobs/{}", id);
    let client = reqwest::Client::new();
    match client.get(endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                return response.text().await.unwrap();
            }
            else {
                return "".to_string();
            }
        }
        Err(_e) => {
            return "".to_string();
        }
    }
}

pub async fn delete_job(id: String) -> bool {
    println!("Deleting job {}", id);
    let endpoint = format!("http://localhost:8181/ws/jobs/{}", id);
    let client = reqwest::Client::new();
    match client.delete(endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                return true;
            }
            else {
                return false;
            }
        }
        Err(_e) => {
            return false;
        }
    }
}

// read the pipeline's web service shutdown key from the temp dir
fn read_shutdown_key() -> Result<String, std::io::Error> {
    // find the shutdown key in the temp dir
    let tmpdir = env::temp_dir();
    let f = File::open(tmpdir.join("dp2key.txt"));
    println!("Reading shutdown key from {}", tmpdir.display());

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// fn read_file_to_buffer(filename: &str) -> Result<(std::vec::Vec<u8>, u64), std::io::Error> {
//     let mut f = File::open(filename)?;
//     let mut buffer = Vec::new();

//     // read the whole file
//     let sz = f.read_to_end(&mut buffer)?;

//     Ok((buffer, u64::try_from(sz).unwrap()))

// }