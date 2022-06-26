use std::fmt;

#[derive(Debug)]
pub enum PipelineServiceError {
    WontStart,
    TimedOut
}

impl std::error::Error for PipelineServiceError {
    fn description(&self) -> &str {
        match *self {
            PipelineServiceError::WontStart => "Process could not be started",
            PipelineServiceError::TimedOut => "Timed out before process could finish starting",
        }
    }
}
impl fmt::Display for PipelineServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pipeline Service unavailable")
    }
}
