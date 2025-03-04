/**
 * This file will automatically be loaded by webpack and run in the "renderer" context.
 * To learn more about the differences between the "main" and the "renderer" context in
 * Electron, visit:
 *
 * https://electronjs.org/docs/tutorial/process-model
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

import mediaRecorder from '@mono-repo/echo_module'

mediaRecorder.initialise(function (e) {
  requestAnimationFrame(() => {
    
    switch (e.event_type) {
      case "new_energy":
        //     appendTranscription(transcription);
        //updateEnergyDisplay(e.message)
        break;
        
        case "transcription":
        console.log(e);
        appendTranscription(e.message, e.event_id);
        break;
    }

  });

});

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


//silence_threshold: number, duration_threshhold: number
var silence_threshold = 1
var duration_threshhold = 25;
async function startRecording() {
  mediaRecorder.start(silence_threshold, duration_threshhold);
}


async function stopRecording() {
  mediaRecorder.stop()
}

function appendTranscription(transcription, event_id) {
  const list = document.getElementById('transcriptionList');
  let existingItem = document.getElementById(event_id);

  if (existingItem) {
    // Element with event_id exists, update its textContent
    existingItem.textContent = transcription;
  } else {
    // Element with event_id does not exist, create a new one
    const newItem = document.createElement('div');
    newItem.className = 'transcriptionItem';
    newItem.id = event_id; // Set the id to the event_id
    newItem.textContent = transcription;

    list.appendChild(newItem);
  }
}

// //TODO set this as a call back in Neon rs

// setInterval(function () {
//   let transcription = mediaRecorder.get();

//   transcription.forEach(e => {
//     appendTranscription(transcription);
//   })

//   mediaRecorder.clear();

// }, 1000);

// //TODO set this as a call back in Neon rs

// setInterval(function () {
//   let energy = mediaRecorder.get_energy();
//   updateEnergyDisplay(energy)


// }, 500);



function updateEnergyDisplay(energy) {
  const displayElement = document.getElementById("energyDisplay");
  if (displayElement) {
    if (energy !== null) {
      displayElement.textContent = "Energy: " + energy
    } else {
      displayElement.textContent = "Energy: Error";
    }
  }
}

function sliderValueChanged(sliderElement) {
  const value = parseFloat(sliderElement.value);
  const sliderId = sliderElement.id;
  const displayElementId = sliderId + 'Value';
  const displayElement = document.getElementById(displayElementId);

  if (displayElement) {
    displayElement.textContent = `${sliderId}:` + value;
  }

  if (displayElementId == "AudioVolumeThreshold") {
    silence_threshold = value;
  }

  else {
    duration_threshhold = value;
  }

}

const slider = document.getElementById('AudioVolumeThreshold');
const slider1 = document.getElementById('PauseTime');

slider.addEventListener('input', function () {
  sliderValueChanged(this);
});

slider1.addEventListener('input', function () {
  sliderValueChanged(this);
});