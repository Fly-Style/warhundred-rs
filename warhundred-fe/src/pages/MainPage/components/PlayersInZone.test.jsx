import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import { PlayersInZone } from './PlayersInZone';
import axios from 'axios';

// Mock axios
vi.mock('axios');

describe('PlayersInZone', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders with test data when useTestData is true', () => {
    // Arrange & Act
    render(<PlayersInZone useTestData={true} />);

    // Assert
    expect(screen.getByText('Players in Zone')).toBeInTheDocument();
    expect(screen.getByText('5 online')).toBeInTheDocument();

    // Check if all test players are rendered with the correct format
    expect(screen.getByText('Warrior123[7]')).toBeInTheDocument();
    expect(screen.getByText('ArcherQueen[5]')).toBeInTheDocument();
    expect(screen.getByText('DarkWizard[6]')).toBeInTheDocument();
    expect(screen.getByText('HealerGirl[3]')).toBeInTheDocument();
    expect(screen.getByText('TankMaster[7]')).toBeInTheDocument();

    // Check for avatars
    const avatars = screen.getAllByRole('generic', { name: '' }).filter(
      element => element.className.includes('player-avatar')
    );
    expect(avatars.length).toBe(5);
    expect(avatars[0]).toHaveTextContent('W'); // First letter of Warrior123

    // Check for action buttons
    const messageButtons = screen.getAllByTitle('Send message');
    const profileButtons = screen.getAllByTitle('View profile');
    expect(messageButtons.length).toBe(5);
    expect(profileButtons.length).toBe(5);
  });

  it('shows loading state when fetching data', async () => {
    // Arrange
    axios.get.mockImplementation(() => new Promise(() => {
      // This promise never resolves, keeping the component in loading state
      setTimeout(() => {}, 1000);
    }));

    // Act
    render(<PlayersInZone />);

    // Assert
    expect(screen.getByText('Loading players...')).toBeInTheDocument();
  });

  it('shows error message when API call fails', async () => {
    // Arrange
    axios.get.mockRejectedValue(new Error('Network error'));

    // Act
    render(<PlayersInZone />);

    // Assert
    await waitFor(() => {
      expect(screen.getByText('Error connecting to server')).toBeInTheDocument();
    });
  });

  it('shows empty state when no players are returned', async () => {
    // Arrange
    axios.get.mockResolvedValue({
      status: 200,
      data: []
    });

    // Act
    render(<PlayersInZone />);

    // Assert
    await waitFor(() => {
      expect(screen.getByText('No players in this zone')).toBeInTheDocument();
    });
  });

  it('fetches players data on mount and sets up interval', async () => {
    // Arrange
    const mockPlayers = [
      { id: 1, nickname: 'TestPlayer', level: 4 }
    ];

    axios.get.mockResolvedValue({
      status: 200,
      data: mockPlayers
    });

    // Act
    render(<PlayersInZone />);

    // Assert
    expect(axios.get).toHaveBeenCalledWith('/zone/players');

    await waitFor(() => {
      expect(screen.getByText('TestPlayer[4]')).toBeInTheDocument();
    });

    // Check that setInterval was called (indirectly by checking if axios.get was called)
    expect(axios.get).toHaveBeenCalledTimes(1);

    // We can't easily test the interval directly, but we've verified the initial call
  });

  it('cleans up interval on unmount', () => {
    // Arrange
    // Mock axios to return a resolved promise so the interval is set up
    axios.get.mockResolvedValue({
      status: 200,
      data: []
    });

    const clearIntervalSpy = vi.spyOn(window, 'clearInterval');
    const { unmount } = render(<PlayersInZone useTestData={false} />);

    // Act
    unmount();

    // Assert
    expect(clearIntervalSpy).toHaveBeenCalled();

    // Clean up
    clearIntervalSpy.mockRestore();
  });
});
