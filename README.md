# DAISY Pipeline UI Prototype: Tauri

This prototype uses [Tauri](https://tauri.studio/) and is written in Rust and Javascript.

The goal of the prototype is to experiment with the stack and demonstrate integration with the Pipeline via executing jobs. 

To run it:

* Follow the [getting started](https://tauri.studio/v1/guides/getting-started/prerequisites) guide
* `npm i`
* `npm run dev`

## Current status

The pipeline web service starts when the app opens and stops when the app closes.

Start a pre-configured job by pressing the button. Each submitted job is opened in a tab.

A web worker polls the Pipeline WS API every 2 seconds, and updates the UI accordingly.

The included Pipeline build is for mac; Set the value of `DAISY_PIPELINE` in `src-tauri/.env` for other systems.

## Observations

The Pipeline WS cannot be called directly from the webview because of CORS issues. Inter-process communication is required to make WS calls (so: they happen in the Rust layer).

## Ideas scratchpad

Ideas for user preferences:

* Clear pipeline log
* Help/how to file a bug
* Clear job history
* Set job output folder
* Set pipeline runtime location 
