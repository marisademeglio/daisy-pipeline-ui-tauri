onmessage = e => {
    if (e.data.id == "START_JOB") {
        setTimeout(() => {
            postMessage({id: "JOB_DONE"});
        }, e.data.dur);   
        
        let interval = e.data.dur/6;
        
        setTimeout(() => {
            postMessage({id: "JOB_MESSAGE", msg: "Job started"});
        }, interval);

        setTimeout(() => {
            postMessage({id: "JOB_MESSAGE", msg: "Parsing files..."});
        }, interval * 2);

        setTimeout(() => {
            postMessage({id: "JOB_MESSAGE", msg: "Transforming content..."});
        }, interval * 3);

        setTimeout(() => {
            postMessage({id: "JOB_MESSAGE", msg: "Copying resources..."});
        }, interval * 4);

        setTimeout(() => {
            postMessage({id: "JOB_MESSAGE", msg: "Job done"});
        }, interval * 5);

    }
};
