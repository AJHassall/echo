/**
 * This file will automatically be loaded by webpack and run in the "renderer" context.
 * To learn more about the differences between the "main" and the "renderer" context in
 * Electron, visit:
 *
 * https://electronjs.org/docs/tutorial/application-architecture#main-and-renderer-processes
 *
 * By default, Node.js integration in this file is disabled. When enabling Node.js integration
 * in a renderer process, please be aware of potential security implications. You can read
 * more about security risks here:
 *
 * https://electronjs.org/docs/tutorial/security
 *
 * To enable Node.js integration in this file, open up `main.js` and enable the `nodeIntegration`
 * flag:
 *
 * ```
 *  // Create the browser window.
 *  mainWindow = new BrowserWindow({
 *    width: 800,
 *    height: 600,
 *    webPreferences: {
 *      nodeIntegration: true
 *    }
 *  });
 * ```
 */

import './index.css';

console.log('👋 This message is being logged by "renderer.js", included via webpack');

import { ipcRenderer } from 'electron';
import mediaRecorder from 'echo_transcriber'
let recordedChunks = [];

console.log(mediaRecorder);

mediaRecorder.initialise();

// Buttons
const videoElement = document.querySelector('video');

const startBtn = document.getElementById('startBtn');
startBtn.onclick = e => {
  startRecording();
  startBtn.innerText = 'Recording';
};

const stopBtn = document.getElementById('stopBtn');

stopBtn.onclick = e => {
  stopRecording();
  startBtn.innerText = 'Start';
};


async function getVideoSources() {
  const inputSources = await ipcRenderer.invoke('getSources')

  inputSources.forEach(source => {
    const element = document.createElement("option")
    element.value = source.id
    element.innerHTML = source.name
    selectMenu.appendChild(element)
  });
}


async function startRecording() {
  mediaRecorder.start();
}


async function stopRecording() {
  mediaRecorder.stop()
}

setInterval(function(){ 
  let transcription = mediaRecorder.get();


  
  transcription.forEach(e=>{
    console.log(e);
  })


  mediaRecorder.clear();
  
  }, 1000);