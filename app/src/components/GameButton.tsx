import React from "react";

import "./GameButton.css";

interface GameButtonProps {
  children: any;
  onClick: any;
}

export default function GameButton(props: GameButtonProps) {
  return (
    <div className="game-btn-container" onClick={(e) => props.onClick(e)}>
      <div className="game-btn-container-left">&nbsp;</div>
      <div className="game-btn-container-center">
        <button className="game-btn-button">
          {props.children}
        </button>
      </div>
      <div className="game-btn-container-right">&nbsp;</div>
    </div>
  );
}
