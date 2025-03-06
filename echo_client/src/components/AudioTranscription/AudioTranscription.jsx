import React, { useState, useEffect, useRef, useContext } from 'react';
import { AudioContext } from '../../contexts/AudioContext';

const AudioTranscription = ({ mediaRecorder }) => {
  const [transcriptions, setTranscriptions] = useState({});
  const animationFrameRef = useRef(null);
  const [isRecording, setIsRecording] = useState(false);
  const { selectedAudioSources } = useContext(AudioContext);

  useEffect(() => {
    if (!mediaRecorder) return;

    const initialiseCallback = (e) => {
      animationFrameRef.current = requestAnimationFrame(() => {
        switch (e.event_type) {
          case 'transcription':
            setTranscriptions((prev) => {
              const updatedTranscriptions = { ...prev };
              if (updatedTranscriptions[e.event_id]) {
                updatedTranscriptions[e.event_id] += " " + e.message;
              } else {
                updatedTranscriptions[e.event_id] = e.message;
              }
              return updatedTranscriptions;
            });
            break;
          case 'recording_started':
            setIsRecording(true);
            break;
          case 'recording_stopped':
            setIsRecording(false);
            break;
          default:
            break;
        }
      });
    };

    mediaRecorder.initialise(initialiseCallback);

    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [mediaRecorder]);

  const handleStartRecording = () => {
    if (mediaRecorder && !isRecording) {
      mediaRecorder.start(selectedAudioSources, -1);
      setIsRecording(true);
    }
  };

  const handleStopRecording = () => {
    if (mediaRecorder && isRecording) {
      setIsRecording(false);
      mediaRecorder.stop();
    }
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100vh', fontFamily: 'Arial, sans-serif', padding: '20px', border: '1px solid #ccc', borderRadius: '8px' }}>
      <h2>Audio Transcription</h2>
      <div style={{ marginBottom: '15px' }}>
        <button
          onClick={handleStartRecording}
          disabled={isRecording}
          style={{
            padding: '10px 15px',
            backgroundColor: isRecording ? '#ddd' : '#4CAF50',
            color: 'white',
            border: 'none',
            borderRadius: '4px',
            cursor: isRecording ? 'not-allowed' : 'pointer',
            marginRight: '10px',
          }}
        >
          {'Start Recording'}
        </button>
        <button
          onClick={handleStopRecording}
          disabled={!isRecording}
          style={{
            padding: '10px 15px',
            backgroundColor: !isRecording ? '#ddd' : '#f44336',
            color: 'white',
            border: 'none',
            borderRadius: '4px',
            cursor: !isRecording && 'not-allowed',
          }}
        >
          {'Stop Recording'}
        </button>
      </div>

      <div style={{ flex: 1, overflowY: 'auto', border: '1px solid #eee', padding: '10px', backgroundColor: '#f9f9f9', borderRadius: '4px' }}>
        {Object.values(transcriptions).map((transcription, index) => (
          <p key={index} style={{ marginBottom: '10px', paddingBottom: '10px', borderBottom: '1px solid #ddd', wordWrap: 'break-word' }}>
            {transcription}
          </p>
        ))}
      </div>
    </div>
  );
};

export default AudioTranscription;