use std::sync::Mutex;

pub struct Jobs(pub Mutex<Vec<Mutex<Job>>>);

pub struct Job {
    pub id: String,
    pub status: String
}
