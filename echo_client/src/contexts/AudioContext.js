import React, { createContext, useState } from 'react';

export const AudioContext = createContext();

export const AudioProvider = ({ children }) => {
  const [selectedAudioSources, setSelectedAudioSources] = useState([]);

  const value = { // Corrected: Added 'value' prop
    selectedAudioSources,
    setSelectedAudioSources,
  };

  return (
    <AudioContext.Provider value={{ selectedAudioSources, setSelectedAudioSources }}>
      {children}
    </AudioContext.Provider>
  );
};