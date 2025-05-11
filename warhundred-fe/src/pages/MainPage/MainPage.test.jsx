import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { MainPage } from './MainPage';

// Mock the useAuth hook before importing it
vi.mock('../../context/AuthProvider.jsx', () => ({
  useAuth: vi.fn()
}));

// Import after mocking
import { useAuth } from '../../context/AuthProvider';

// Disable prop-types validation for mock components
/* eslint-disable react/prop-types */

// Mock all components and hooks
vi.mock('./components/Chat.jsx', () => ({
  Chat: ({ children }) => <div data-testid="chat">{children}</div>
}));

vi.mock('./components/PlayersInZone.jsx', () => ({
  PlayersInZone: ({ children, useTestData }) => <div data-testid="players-in-zone" data-use-test-data={useTestData}>{children}</div>
}));

vi.mock('../GameWindow/GameWindow.jsx', () => ({
  default: ({ className }) => <div data-testid="game-window" className={className}>Game Window</div>
}));

// Mock the Button component
vi.mock('../../components/ui', () => ({
  Button: ({ children, buttonProps, variant, className }) => (
    <button 
      data-testid="button" 
      onClick={buttonProps?.onClick}
      className={className}
      data-variant={variant}
    >
      {children}
    </button>
  )
}));

/* eslint-enable react/prop-types */

describe('MainPage', () => {
  it('renders correctly with all child components', () => {
    // Arrange
    const mockAuth = {
      logout: vi.fn()
    };
    useAuth.mockReturnValue(mockAuth);

    // Act
    render(<MainPage />);

    // Assert
    expect(screen.getByText('Header')).toBeInTheDocument();
    expect(screen.getByTestId('game-window')).toBeInTheDocument();
    expect(screen.getByText('Logout')).toBeInTheDocument();
    expect(screen.getByText('LOCATIONS')).toBeInTheDocument();
    expect(screen.getByTestId('chat')).toBeInTheDocument();
    expect(screen.getByTestId('players-in-zone')).toBeInTheDocument();
  });

  it('calls auth.logout when logout button is clicked', () => {
    // Arrange
    const mockAuth = {
      logout: vi.fn()
    };
    useAuth.mockReturnValue(mockAuth);

    // Act
    render(<MainPage />);

    // Find and click the logout button
    const logoutButton = screen.getByText('Logout');
    fireEvent.click(logoutButton);

    // Assert
    expect(mockAuth.logout).toHaveBeenCalledTimes(1);
  });

  it('renders the logout button with correct props', () => {
    // Arrange
    const mockAuth = {
      logout: vi.fn()
    };
    useAuth.mockReturnValue(mockAuth);

    // Act
    render(<MainPage />);

    // Assert
    const button = screen.getByTestId('button');
    expect(button).toHaveTextContent('Logout');
    expect(button).toHaveAttribute('data-variant', 'danger');
    expect(button).toHaveClass('logout-btn');
  });

  it('renders the game window with correct class name', () => {
    // Arrange
    const mockAuth = {
      logout: vi.fn()
    };
    useAuth.mockReturnValue(mockAuth);

    // Act
    render(<MainPage />);

    // Assert
    const gameWindow = screen.getByTestId('game-window');
    expect(gameWindow).toHaveClass('cnt');
    expect(gameWindow).toHaveClass('game-window-wrap');
  });
});
