import { createContext, useContext, useState } from "react";
import PropTypes from 'prop-types';

// Define game states - not a component, so it's exempt from the react-refresh/only-export-components rule
/* eslint-disable react-refresh/only-export-components */
export const GameStates = {
  TOWN: "TOWN",
  BATTLE: "BATTLE",
};
/* eslint-enable react-refresh/only-export-components */

// Create context
const GameStateContext = createContext();

/**
 * Provider component for game state
 * @param {Object} props - Component props
 * @param {React.ReactNode} props.children - Child components
 * @returns {JSX.Element} - Provider component
 */
export const GameStateProvider = ({ children }) => {
  const [gameState, setGameState] = useState(GameStates.TOWN);

  /**
   * Change the game state
   * @param {string} newState - New game state
   */
  const changeGameState = (newState) => {
    if (Object.values(GameStates).includes(newState)) {
      setGameState(newState);
    } else {
      console.error(`Invalid game state: ${newState}`);
    }
  };

  // Value object to be provided to consumers
  const value = {
    gameState,
    changeGameState,
  };

  return (
    <GameStateContext.Provider value={value}>
      {children}
    </GameStateContext.Provider>
  );
};

GameStateProvider.propTypes = {
  children: PropTypes.node.isRequired
};

/**
 * Hook to use the game state context - not a component, so it's exempt from the react-refresh/only-export-components rule
 * @returns {Object} - Game state context value
 */
/* eslint-disable react-refresh/only-export-components */
export const useGameState = () => {
  const context = useContext(GameStateContext);
  if (context === undefined) {
    throw new Error("useGameState must be used within a GameStateProvider");
  }
  return context;
};
/* eslint-enable react-refresh/only-export-components */

export default GameStateContext;
