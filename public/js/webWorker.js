// the worker doesn't have access to the pipeline web service
// so it just waits 5 seconds and tells the main thread to get the data
// then when the main thread receives the message from the worker, it tells the worker to restart the timer,
// and the whole thing repeats
onmessage = e => {
    if (e.data == "START_TIMER") {
        setTimeout(() => {
            postMessage("TIMER_EVENT");
        }, 2000);   
    }
};
