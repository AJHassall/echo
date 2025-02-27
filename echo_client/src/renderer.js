import './index.css';

import { ipcRenderer } from 'electron';
import mediaRecorder from 'echo_transcriber'

mediaRecorder.initialise();

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
  mediaRecorder.start(silence_threshold, duration_threshhold, function(e){console.log(e)});
}


async function stopRecording() {
  mediaRecorder.stop()
}

function appendTranscription(transcription) {
  const list = document.getElementById('transcriptionList');
  const newItem = document.createElement('div');
  newItem.className = 'transcriptionItem';
  newItem.textContent = transcription;
  list.appendChild(newItem);
}

//TODO set this as a call back in Neon rs

setInterval(function () {
  let transcription = mediaRecorder.get();

  transcription.forEach(e => {
    appendTranscription(transcription);
  })

  mediaRecorder.clear();

}, 1000);

//TODO set this as a call back in Neon rs

setInterval(function () {
  let energy = mediaRecorder.get_energy();
  updateEnergyDisplay(energy)


}, 500);



function updateEnergyDisplay(energy) {
  const displayElement = document.getElementById("energyDisplay");
  if (displayElement) {
    if (energy !== null) {
      displayElement.textContent = "Energy: " + energy.toFixed(2);
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
