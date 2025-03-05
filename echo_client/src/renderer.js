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
  const selectedSources = getSelectedAudioSources();

  console.log(selectedSources);

  mediaRecorder.start(selectedSources, duration_threshhold);
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
function getAudioSources() {
  return mediaRecorder.get_audio_sources();
}

async function refreshAudioSources() {
  const sources = getAudioSources();
  audioSourceList.innerHTML = '<h3>Audio Sources</h3><button id="refreshAudioSources">Refresh</button>'; // Clear and add refresh button back

  sources.forEach((source) => {
    const label = document.createElement('label');
    const br = document.createElement('br');
    label.appendChild(br);
    const checkbox = document.createElement('input');

    checkbox.type = 'checkbox';
    checkbox.value = source;
    label.appendChild(checkbox);
    label.appendChild(document.createTextNode(source));
    audioSourceList.appendChild(label);
  });

  document.getElementById('refreshAudioSources').addEventListener('click', refreshAudioSources); // Re-add the listener
}

function getSelectedAudioSources() {
  const checkboxes = audioSourceList.querySelectorAll('input[type="checkbox"]:checked');
  return Array.from(checkboxes).map((checkbox) => checkbox.value);
}

refreshAudioSources();


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

import './app.jsx';