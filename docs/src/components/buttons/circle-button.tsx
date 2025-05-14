import React from 'react';

interface CircleButtonProps {
  onClick: () => void;
  children: React.ReactNode;
  className?: string;
  ariaLabel: string;
}

export function CircleButton({ onClick, children, className = '', ariaLabel }: CircleButtonProps) {
  return (
    <button
      onClick={onClick}
      className={`p-2 bg-white rounded-full shadow-lg hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-transform transform hover:scale-110 border-2 border-gray-300 ${className}`}
      aria-label={ariaLabel}
    >
      {children}
    </button>
  );
}
