import React, { useState } from "react";
import TownCanvas from "./components/TownCanvas.jsx";
import BattleCanvas from "./components/BattleCanvas.jsx";
import './GameWindow.css'

const GameStates = {
  TOWN: "TOWN",
  BATTLE: "BATTLE",
};

const GameWindow = () => {
  const [gameState, setGameState] = useState(GameStates.TOWN); // Manage game state

  return (
    <div className="game-window-wrap">
        {gameState === GameStates.TOWN && <TownCanvas />}
        {gameState === GameStates.BATTLE && <BattleCanvas />}
    </div>
  );
};

export default GameWindow;