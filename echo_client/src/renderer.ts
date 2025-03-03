/**
 * This file will automatically be loaded by vite and run in the "renderer" context.
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
 * To enable Node.js integration in this file, open up `main.ts` and enable the `nodeIntegration`
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

console.log('👋 This message is being logged by "renderer.ts", included via Vite');
import mediaRecorder from 'echo_module';

interface EchoModuleEvent  {
    message: string;
    event_id: string;
    event_type: string;
}


const myEventHandler: (event: EchoModuleEvent) => object = (e) => { // Anonymous function assigned to typed variable
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

      return {};
  };

  mediaRecorder.initialise(() => {
    return {}; //  Still using the placeholder for now, as initialise is for setup
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
    
    
    //silence_threshold: number, duration_threshold: number
    let silence_threshold = 1
    let duration_threshold = 25;
    async function startRecording() {
      mediaRecorder.start(silence_threshold, duration_threshold);
    }
    
    
    async function stopRecording() {
      mediaRecorder.stop()
    }
    
    function appendTranscription(transcription: string, event_id: string) {
      const list = document.getElementById('transcriptionList');
      const existingItem = document.getElementById(event_id);
  
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
        
        
        
        function updateEnergyDisplay(energy: string) {
          const displayElement = document.getElementById("energyDisplay");
    if (displayElement) {
      if (energy !== null) {
        displayElement.textContent = "Energy: " + energy
      } else {
        displayElement.textContent = "Energy: Error";
      }
    }
  }
  
  function sliderValueChanged(sliderElement: HTMLElement) {
    const value = parseFloat(sliderElement.textContent);
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
      duration_threshold = value;
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