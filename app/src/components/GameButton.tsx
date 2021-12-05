import React from "react";

import "./GameButton.css";

interface GameButtonProps {
  children: any;
  onClick: any;
}

export default function GameButton(props: GameButtonProps) {
  return (
    <div className="game-btn-container">
      <div className="game-btn-container-left" onClick={(e) => props.onClick(e)}>&nbsp;</div>
      <div className="game-btn-container-center" onClick={(e) => props.onClick(e)}>
        <button className="game-btn-button">
          {props.children}
        </button>
      </div>
      <div className="game-btn-container-right" onClick={(e) => props.onClick(e)}>&nbsp;</div>
    </div>
  );
}
