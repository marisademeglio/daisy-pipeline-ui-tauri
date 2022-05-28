// this all assumes there's one row of tabs and they are all part of the same set
// we could have multiple sets of tabs (maybe?) and if so, adjustments are required here

let getAllTabs = () => Array.from(document.querySelectorAll("button[role=tab]"));
let getAllTabPanels = () => Array.from(document.querySelectorAll("div[role=tabpanel]"));

// select one tab and deselect all the others
function displayTab(tabElement) {
    // use aria-selected=true/false on the button
    // and class=selected on the corresponding panel
    console.log(`Selecting tab ${tabElement.id}`);

    let tabs = getAllTabs();
    tabs.map(tab => tab.setAttribute("aria-selected", tab.id == tabElement.id));

    let panels = getAllTabPanels();
    panels.map(panel => panel.getAttribute('aria-labelledby') == tabElement.id ? 
        panel.classList.add("selected") 
        : 
        panel.classList.remove("selected"));    
}

function addTab(id) {
    let tabElement = document.createElement("button");
    tabElement.setAttribute("id", `job-${id}`);
    tabElement.setAttribute("tab-index", "0");
    tabElement.setAttribute("aria-selected", "false");
    tabElement.setAttribute("role", "tab");
    tabElement.setAttribute("data-jobid", id);
    tabElement.setAttribute("aria-live", "polite");

    let panelElement = document.createElement("div");
    panelElement.setAttribute("role", "tabpanel");
    panelElement.setAttribute("aria-labelledby", `job-${id}`);
    panelElement.setAttribute("aria-live", "polite");

    document.querySelector("#tabs").appendChild(tabElement);
    document.querySelector("main").appendChild(panelElement);
    tabElement.addEventListener('click', e => {
        displayTab(tabElement);
    });
    return {tab: tabElement, panel: panelElement};
}

function getTabPanel(jobid) {
    // first get the tab for the job
    let tab = document.querySelector(`button[role=tab][data-jobid='${jobid}']`);
    // then get the panel that is labelled by that button
    let panel = document.querySelector(`div[role=tabpanel][aria-labelledby='${tab.id}']`)
    return panel;
}

export { displayTab, addTab, removeTab, getTabPanel };