import React, { useState, useEffect, useContext } from 'react';
import { AudioContext } from '../../contexts/AudioContext';

const AudioSourceSelector = ({ mediaRecorder }) => {
    const [audioSources, setAudioSources] = useState([]);
    const { setSelectedAudioSources, selectedAudioSources } = useContext(AudioContext);

    const getAudioSources = () => {
        if (mediaRecorder) {
            mediaRecorder.get_audio_sources().then(sources => {
                setAudioSources(sources);
            });
        }
    };

    const handleCheckboxChange = (source) => {
        setSelectedAudioSources((prevSelectedSources) => {
            if (prevSelectedSources.includes(source)) {
                return prevSelectedSources.filter((s) => s !== source);
            } else {
                return [...prevSelectedSources, source];
            }
        });
    };

    useEffect(() => {
        if (mediaRecorder) {
            getAudioSources();
        }
    }, [mediaRecorder]);

    return (
        <div style={{ padding: '10px' }}>
            <button
                onClick={getAudioSources}
                style={{
                    padding: '8px 12px',
                    backgroundColor: '#007bff',
                    color: 'white',
                    border: 'none',
                    borderRadius: '4px',
                    cursor: 'pointer',
                    marginBottom: '10px',
                }}
            >
                Refresh Audio Sources
            </button>
            {audioSources.length > 0 && (
                <div>
                    <h3>Audio Sources:</h3>
                    <ul style={{ listStyle: 'none', padding: 0 }}>
                        {audioSources.map((source) => (
                            <li key={source} style={{ marginBottom: '5px' }}>
                                <label style={{ display: 'flex', alignItems: 'center' }}>
                                    <input
                                        type="checkbox"
                                        checked={selectedAudioSources?.includes(source)}
                                        onChange={() => handleCheckboxChange(source)}
                                        style={{ marginRight: '5px' }}
                                    />
                                    {source}
                                </label>
                            </li>
                        ))}
                    </ul>
                </div>
            )}
        </div>
    );
};

export default AudioSourceSelector;