import React, { useState, useRef, useEffect } from 'react';
import './SliderPanel.css';

const SliderPanel = ({ children, edgeSize = 50, panelWidth = 300, side = 'left' }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [isHovering, setIsHovering] = useState(false);
  const isHoveringRef = useRef(false);
  const panelRef = useRef(null);

  const handleMouseMove = (e) => {
    const windowWidth = window.innerWidth;
    if (!isOpen) {
      if (side === 'left' && e.clientX <= edgeSize) {
        setIsOpen(true);
      } else if (side === 'right' && e.clientX >= windowWidth - edgeSize) {
        setIsOpen(true);
      }
    }
  };

  const handleMouseEnter = () => {
    setIsHovering(true);
    isHoveringRef.current = true;
  };

  const handleMouseLeave = () => {
    setIsHovering(false);
    isHoveringRef.current = false;
    setTimeout(() => {
      if (isOpen && !isHoveringRef.current) {
        setIsOpen(false);
      }
    }, 100);
  };

  useEffect(() => {
    window.addEventListener('mousemove', handleMouseMove);
    return () => {
      window.removeEventListener('mousemove', handleMouseMove);
    };
  }, [isOpen, edgeSize, side]); // Add side to useEffect dependencies

  return (
    <div
      ref={panelRef}
      className={`slider-panel ${isOpen ? 'open' : ''} ${side}`}
      style={{ width: panelWidth }}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    >
      {children}
    </div>
  );
};

export default SliderPanel;