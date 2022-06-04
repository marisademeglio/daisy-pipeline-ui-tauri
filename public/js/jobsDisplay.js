import * as tabs from './tabs.js';

let nsResolver = () => 'http://www.daisy.org/ns/pipeline/data';

async function update(jobsXmlString, getJobDataFn, deleteJobFn) {
    // are there jobs running
    const parser = new DOMParser();
    const xmldoc = parser.parseFromString(jobsXmlString, "application/xml");
    
    let jobsIterator = xmldoc.evaluate("//ns:job", xmldoc.documentElement, nsResolver, XPathExpression.ANY_TYPE, null);
    let job = jobsIterator.iterateNext();
    while(job) {
        // if the job is done and the UI already has reported this as its status, don't update anything
        let status = job.getAttribute("status");
        let res = tabs.getTabAndTabPanel(job.getAttribute("id"));
        let panel = res.panel;
        let currentStatus = getStatusFromPanel(panel); // get the currently-displayed status
        if (status != "RUNNING" && status == currentStatus) {
            console.log("No change in job data, no need to update UI");
        }
        else {
            await createOrUpdateJob(job.getAttribute("id"), getJobDataFn, deleteJobFn);
        }
        if (currentStatus != status && currentStatus != "") {
            window.__TAURI__.notification.sendNotification({title: "Pipeline job", body: status});
        }
        job = jobsIterator.iterateNext();
    }
}
async function createOrUpdateJob(jobId, getJobDataFn, deleteJobFn) {
    let jobXml = await getJobDataFn(jobId);
    const parser = new DOMParser();
    const xmldoc = parser.parseFromString(jobXml, "application/xml");

    let id = xmldoc.evaluate("//ns:job/@id", xmldoc.documentElement, nsResolver, XPathExpression.ANY_TYPE, null).iterateNext().nodeValue;
    
    // see if this job exists in our UI yet
    let res = tabs.getTabAndTabPanel(id);
    let tab = res.tab;
    let panel = res.panel;

    // if not, create it
    if (!tab || !panel) {
        console.log("Creating tab for job");
        res = tabs.addTab(id);
        tab = res.tab;
        panel = res.panel;
    }
    
    let status = xmldoc.evaluate("//ns:job/@status", xmldoc.documentElement, nsResolver, XPathExpression.ANY_TYPE, null).iterateNext().nodeValue;
    let name = xmldoc.evaluate("//ns:script/ns:nicename/text()", xmldoc.documentElement, nsResolver, XPathExpression.ANY_TYPE, null).iterateNext().nodeValue;
    
    let result = status == "SUCCESS" ? 
        xmldoc.evaluate("//ns:result[@file]/@file", xmldoc.documentElement, nsResolver, XPathExpression.ANY_TYPE, null).iterateNext().nodeValue
        : 
        "Not available";
    let messagesIt = xmldoc.evaluate("//ns:message/@content", xmldoc.documentElement, nsResolver, XPathEvaluator.ANY_TYPE, null);
    let msg = messagesIt.iterateNext();
    let messages = [];
    while (msg) {
        messages.push(msg.nodeValue);
        msg = messagesIt.iterateNext();
    }
    
    tab.innerHTML = `
    <span class="job-name">${name}</span>
    <span class="job-status ${status}">${status.toLowerCase()}</span>`;

    panel.innerHTML = `
    <h2>${name}</h2>
    <ul class="status">
        <li>
            <span>ID</span>
            <span>${id}</span>
        </li>
        <li>
            <span>Status</span>
            <span>${status}</span>
        </li>
        <li>
            <span>Result</span>
            <span>${result}</span>
        </li>
    </ul>
    
    ${status != 'RUNNING' ? `<button class="delete-job">Delete job</button>` : ``}

    <div role="region" aria-labelledby="messages" tabindex="0" class="messages-container">
        <h3>Messages</h3>

        <ul class="messages">
        ${messages.map(m => `<li>${m}</li>`).join('')}
        </ul>
    </div>
    `;

    if (status != 'RUNNING') {
        panel.querySelector("button.delete-job").addEventListener('click', async e => {
            await deleteJobFn(id);

            // focus on the next tab
            let nextTab = tab.nextElementSibling;
            if (!nextTab) {
                nextTab = document.querySelector("#start");
            }
            tab.remove();
            panel.remove();
            tabs.selectTab(nextTab);
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