// this keeps track of all the jobs
// it's just a mockup; see the main github branch for actual pipeline integration

// job definition
/*
    {
        id,
        scriptName,
        status,
        results,
        messages,
        displayed
    }
*/

let jobs = [];

function addJob(scriptName) {
    let jobData = {
        id: crypto.randomUUID(),
        scriptName,
        status: "RUNNING",
        results: "",
        messages: [],
        displayed: true
    };
    jobs.push(jobData);
    return jobData;
}
function removeJob(id) {
    let idx = jobs.findIndex(j => j.id == id);
    jobs.splice(idx, 1);
}
function getJobs() {
    return jobs;
}
function getJob(id) {
    return jobs.find(j => j.id == id);
}
function updateJobStatus(jobId, status) {
    let job = jobs.find(j => j.id == jobId);
    job.status = status;
    if (status == "SUCCESS") {
        job.results = "file:///path/to/output";
    }
    return job;
}
function addMessageToJob(jobId, message) {
    let job = jobs.find(j => j.id == jobId);
    job.messages.push(message);
    return job;
}
function updateJobDisplayed(jobId, displayed) {
    let job = jobs.find(j => j.id == jobId);
    job.displayed = displayed;
    return job;
}

export { getJob, getJobs, addJob, removeJob, updateJobStatus, addMessageToJob, updateJobDisplayed }