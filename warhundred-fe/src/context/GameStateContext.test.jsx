import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { GameStateProvider, useGameState, GameStates } from './GameStateContext';

// Test component that uses the game state context
const TestComponent = () => {
  const { gameState, changeGameState } = useGameState();
  return (
    <div>
      <div data-testid="game-state">{gameState}</div>
      <button onClick={() => changeGameState(GameStates.TOWN)}>Set Town</button>
      <button onClick={() => changeGameState(GameStates.BATTLE)}>Set Battle</button>
      <button onClick={() => changeGameState('INVALID')}>Set Invalid</button>
    </div>
  );
};

describe('GameStateContext', () => {
  beforeEach(() => {
    // Spy on console.error to test error logging
    vi.spyOn(console, 'error').mockImplementation(() => {});
  });

  it('renders children correctly', () => {
    // Arrange & Act
    render(
      <GameStateProvider>
        <div data-testid="child">Child content</div>
      </GameStateProvider>
    );

    // Assert
    const child = screen.getByTestId('child');
    expect(child).toBeInTheDocument();
    expect(child).toHaveTextContent('Child content');
  });

  it('provides initial game state value (TOWN)', () => {
    // Arrange & Act
    render(
      <GameStateProvider>
        <TestComponent />
      </GameStateProvider>
    );

    // Assert
    expect(screen.getByTestId('game-state')).toHaveTextContent(GameStates.TOWN);
  });

  it('changes game state to BATTLE when button is clicked', () => {
    // Arrange & Act
    render(
      <GameStateProvider>
        <TestComponent />
      </GameStateProvider>
    );

    // Initial state should be TOWN
    expect(screen.getByTestId('game-state')).toHaveTextContent(GameStates.TOWN);

    // Click the button to change state to BATTLE
    const battleButton = screen.getByText('Set Battle');
    fireEvent.click(battleButton);

    // Assert
    expect(screen.getByTestId('game-state')).toHaveTextContent(GameStates.BATTLE);
  });

  it('changes game state back to TOWN when button is clicked', () => {
    // Arrange & Act
    render(
      <GameStateProvider>
        <TestComponent />
      </GameStateProvider>
    );

    // First change to BATTLE
    const battleButton = screen.getByText('Set Battle');
    fireEvent.click(battleButton);
    expect(screen.getByTestId('game-state')).toHaveTextContent(GameStates.BATTLE);

    // Then change back to TOWN
    const townButton = screen.getByText('Set Town');
    fireEvent.click(townButton);

    // Assert
    expect(screen.getByTestId('game-state')).toHaveTextContent(GameStates.TOWN);
  });

  it('logs error and does not change state when invalid state is provided', () => {
    // Arrange & Act
    render(
      <GameStateProvider>
        <TestComponent />
      </GameStateProvider>
    );

    // Initial state should be TOWN
    expect(screen.getByTestId('game-state')).toHaveTextContent(GameStates.TOWN);

    // Click the button to try to change state to an invalid value
    const invalidButton = screen.getByText('Set Invalid');
    fireEvent.click(invalidButton);

    // Assert
    expect(screen.getByTestId('game-state')).toHaveTextContent(GameStates.TOWN); // State should not change
    expect(console.error).toHaveBeenCalledWith('Invalid game state: INVALID');
  });

  it('throws error when useGameState is used outside of GameStateProvider', () => {
    // Arrange
    const ErrorComponent = () => {
      useGameState(); // This should throw an error
      return null;
    };

    // Act & Assert
    expect(() => {
      render(<ErrorComponent />);
    }).toThrow('useGameState must be used within a GameStateProvider');
  });
});
