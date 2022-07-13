import * as jobsList from './jobsList.js';
import * as jobsDisplay from './jobsDisplay.js';

// pretend to run the job with a web worker
function runJob(job) {
    let duration = getRandomInt(2000, 9000);
    let w = new Worker("./js/worker.js");
    w.postMessage({id: "START_JOB", dur: duration});
    w.onmessage = async e => {
        if (e.data.id == "JOB_DONE") {
            jobsList.updateJobStatus(job.id, "SUCCESS");
            await jobsDisplay.update();
        }
        else if (e.data.id == "JOB_MESSAGE") {
            jobsList.addMessageToJob(job.id, e.data.msg);
            await jobsDisplay.update();
        }
    };
    window.onbeforeunload = e => {
        console.log("onbeforeunload");
         w.terminate();
    }

}
function getRandomInt(min, max) {
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min) + min); //The maximum is exclusive and the minimum is inclusive
  }
  
export { runJob };