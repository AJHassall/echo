import React, { useState, useEffect } from "react";
import { Slider } from "./components/VerticalSlider/vertical_slider.jsx";
import SliderPanel from "./components/SliderPanel/SliderPanel.jsx";
import AudioSourceSelector from "./components/AudioSourceSelector/AudioSouorceSelector.jsx";
import AudioTranscription from "./components/AudioTranscription/AudioTranscription.jsx";
import { AudioProvider } from "./contexts/AudioContext.js"; // Import AudioProvider

import mediaRecorder from '@mono-repo/echo_module'


const mockMediaRecorder = {
    initialise: (callback) => {

        mediaRecorder.initialise(callback);


        // setTimeout(() => {
        //     callback({ event_type: "recording_started" });
        // }, 1000);
        // setTimeout(() => {
        //     callback({ event_type: "transcription", message: "Hello, this is a test.", event_id: "1" });
        // }, 2000);
        // setTimeout(() => {
        //     callback({ event_type: "transcription", message: "Hello, this is a test. continues", event_id: "1" });
        // }, 3000);
        // setTimeout(() => {
        //     callback({ event_type: "transcription", message: "Another transcription message.", event_id: "2"  });
        // }, 4000);
        // setTimeout(() => {
        //     callback({ event_type: "recording_stopped" });
        // }, 4500);
    },
    get_audio_sources: async () => {
        return mediaRecorder.get_audio_sources();
    },
    start: (sources, number) => {
        console.log("Recording started (mock)");
        mediaRecorder.start(sources, number);
    },
    stop: () => {
        console.log("Recording stopped (mock)");
        mediaRecorder.stop();
    },
};

const App = () => {
    const [mediaRecorder, setMediaRecorder] = useState(null);

    useEffect(() => {
        setMediaRecorder(mockMediaRecorder);
    }, []);

    return (
        <AudioProvider> {/* Use AudioProvider here */}
            <>
                <SliderPanel edgeSize={30} panelWidth={250} mediaRecorder={mediaRecorder}>
                    <Slider />
                    <Slider />
                </SliderPanel>
                {mediaRecorder && <AudioTranscription mediaRecorder={mediaRecorder} />}
                <SliderPanel side="right" mediaRecorder={mediaRecorder}>
                    {mediaRecorder && <AudioSourceSelector mediaRecorder={mediaRecorder} />}
                </SliderPanel>
            </>
        </AudioProvider>
    );
};

export default App;