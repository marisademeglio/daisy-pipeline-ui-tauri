import * as tabs from './tabs.js';
import * as jobsList from './jobsList.js';

function update() {
    console.log("update");
    let jobsData = jobsList.getJobs();
    console.log(jobsData);
    jobsData.filter(job => job.displayed).map(job => {
        let status = job.status;
        let res = tabs.getTabAndTabPanel(job.id);
        let panel = res.panel;
        let currentStatus = getStatusFromPanel(panel); // get the currently-displayed status
        if (status != "RUNNING" && status == currentStatus) {
            console.log("No change in job data, no need to update UI");
        }
        else {
            updateJob(job.id);
        }
        if (currentStatus != status && currentStatus != "") {
            console.log("Status change");
            window.__TAURI__.notification.sendNotification({title: `Pipeline`, body: `${job.scriptName}: ${status}`});
        }
    });
}
function updateJob(jobId) {
    let job = jobsList.getJob(jobId);
    if (!job) return;

    // see if this job exists in our UI yet
    let res = tabs.getTabAndTabPanel(job.id);
    let tab = res.tab;
    let panel = res.panel;

    // if not, create it
    if (!tab || !panel) {
        console.log("Creating tab for job");
        res = tabs.addTab(job.id);
        tab = res.tab;
        panel = res.panel;
    }
    
    tab.innerHTML = `
    <button class="tab-contents">
        <span class="job-name">${job.scriptName}</span>
        <span class="job-status ${job.status}">${job.status.toLowerCase()}</span>
    </button>`;
    // this is preferable to the "close tab" button in the tab's panel
    // but it doesn't seem accessible
    // ${job.status != 'RUNNING' ? `<button class="close-tab" title="Close tab">x</button>` : ``}`;

    panel.innerHTML = `
    <h2>${job.scriptName}</h2>
    <ul class="status">
        <li>
            <span>ID</span>
            <span>${job.id}</span>
        </li>
        <li>
            <span>Status</span>
            <span class="job-status ${job.status}">${job.status}</span>
        </li>
        <li>
            <span>Result</span>
            <span>${job.results != "" ? job.results : "Pending"}</span>
        </li>
    </ul>
    
    ${job.status != 'RUNNING' ? `<button class="close-tab">Close Tab</button>` : ``}
    ${job.status != 'RUNNING' ? `<button class="delete-job">Delete job</button>` : ``}

    <div role="region" aria-labelledby="messages" tabindex="0" class="messages-container">
        <h3>Messages</h3>

        <ul class="messages">
        ${job.messages.map(m => `<li>${m}</li>`).join('')}
        </ul>
    </div>
    `;

    let closeTab = t => {
        // close tab t and focus on the next tab
        let nextTab = t.nextElementSibling;
        if (!nextTab) {
            nextTab = t.previousElementSibling;
        }
        t.remove();
        panel.remove();
        console.log("closing tab, selecting next", nextTab);
        tabs.selectTab(nextTab);
    };

    if (job.status != 'RUNNING') {
        panel.querySelector("button.delete-job").addEventListener('click', async e => {
            jobsList.removeJob(job.id);
            closeTab(tab);
        });
        panel.querySelector("button.close-tab").addEventListener('click', e => {
            closeTab(tab);
            jobsList.updateJobDisplayed(job.id, false);
            e.stopPropagation(); 
        });
    }
    
}

function getStatusFromPanel(panel) {
    if (!panel) return "";

    let statusElement = panel.querySelector(".job-status");
    if (!statusElement) {
        return "";
    }
    if (statusElement.classList.contains("RUNNING")) {
        return "RUNNING"
    }
    else if (statusElement.classList.contains("IDLE")) {
        return "IDLE";
    }
    else if (statusElement.classList.contains("ERROR")) {
        return "ERROR";
    }
    else if (statusElement.classList.contains("SUCCESS")) {
        return "SUCCESS";
    }
    else {
        return "";
    }
}
export { update };