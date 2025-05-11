import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { MainPage } from './MainPage';
import { useAuth } from '../../context/AuthProvider';
import PropTypes from 'prop-types';

// Mock the child components
vi.mock('./components/Chat.jsx', () => {
  const Chat = ({ children }) => <div data-testid="chat">{children}</div>;
  Chat.propTypes = {
    children: PropTypes.node
  };
  return { Chat };
});

vi.mock('./components/PlayersInZone.jsx', () => {
  const PlayersInZone = ({ children }) => <div data-testid="players-in-zone">{children}</div>;
  PlayersInZone.propTypes = {
    children: PropTypes.node
  };
  return { PlayersInZone };
});

vi.mock('../GameWindow/GameWindow.jsx', () => {
  const GameWindow = ({ className }) => <div data-testid="game-window" className={className}>Game Window</div>;
  GameWindow.propTypes = {
    className: PropTypes.string
  };
  return { default: GameWindow };
});

// Mock the useAuth hook
vi.mock('../../context/AuthProvider.jsx', () => ({
  useAuth: vi.fn()
}));

// Mock the Button component
vi.mock('../../components/ui', () => {
  const Button = ({ children, buttonProps, variant, className }) => (
    <button 
      data-testid="button" 
      onClick={buttonProps?.onClick}
      className={className}
      data-variant={variant}
    >
      {children}
    </button>
  );
  Button.propTypes = {
    children: PropTypes.node,
    buttonProps: PropTypes.shape({
      onClick: PropTypes.func
    }),
    variant: PropTypes.string,
    className: PropTypes.string
  };
  return { Button };
});

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
