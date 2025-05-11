import TownCanvas from "./components/TownCanvas.jsx";
import BattleCanvas from "./components/BattleCanvas.jsx";
import { useGameState, GameStates } from "../../context/GameStateContext.jsx";
import './GameWindow.css'

const GameWindow = () => {
  const { gameState } = useGameState(); // Use global game state

  return (
    <div className="game-window-wrap">
      {gameState === GameStates.TOWN && <TownCanvas/>}
      {gameState === GameStates.BATTLE && <BattleCanvas/>}
    </div>
  );
};

export default GameWindow;
