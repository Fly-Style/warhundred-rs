import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { PlayerMiniProfile } from './PlayerMiniProfile';

describe('PlayerMiniProfile', () => {
  it('renders with test data when useTestData is true', () => {
    // Arrange & Act
    render(<PlayerMiniProfile useTestData={true} />);

    // Assert
    expect(screen.getByText('Your Character')).toBeInTheDocument();

    // Check if player info is rendered
    expect(screen.getByText('YourCharacter')).toBeInTheDocument();
    expect(screen.getByText('Level: 5')).toBeInTheDocument();
    expect(screen.getByText('Rank: Novice')).toBeInTheDocument();

    // Check for stats
    expect(screen.getByText('Health')).toBeInTheDocument();
    expect(screen.getByText('75/100')).toBeInTheDocument();

    expect(screen.getByText('Stamina')).toBeInTheDocument();
    expect(screen.getByText('60/100')).toBeInTheDocument();

    expect(screen.getByText('Experience')).toBeInTheDocument();
    expect(screen.getByText('65%')).toBeInTheDocument();

    // Avatar has been removed from the component

    // Check for stat bars
    const healthBar = document.querySelector('.health-bar');
    const staminaBar = document.querySelector('.stamina-bar');
    const experienceBar = document.querySelector('.experience-bar');

    expect(healthBar).toHaveStyle('width: 75%');
    expect(staminaBar).toHaveStyle('width: 60%');
    expect(experienceBar).toHaveStyle('width: 65%');
  });

  it('shows loading state when no data is available', () => {
    // Arrange & Act
    render(<PlayerMiniProfile useTestData={false} />);

    // Assert
    expect(screen.getByText('Loading player data...')).toBeInTheDocument();
  });
});
