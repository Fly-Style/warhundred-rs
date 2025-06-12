import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import RequireAuth from './RequireAuth';
import { useAuth } from '../../context/AuthProvider';

// Mock the MainPage and EntryPage components
vi.mock('../../pages/MainPage/MainPage.jsx', () => ({
  MainPage: () => <div data-testid="main-page">Main Page</div>
}));

vi.mock('../../pages/EntryPage/EntryPage.jsx', () => ({
  default: () => <div data-testid="entry-page">Entry Page</div>
}));

// Mock the useAuth hook
vi.mock('../../context/AuthProvider.jsx', () => ({
  useAuth: vi.fn()
}));

describe('RequireAuth', () => {
  it('renders EntryPage when user is not authenticated', () => {
    // Arrange
    useAuth.mockReturnValue({ user: null });

    // Act
    render(<RequireAuth />);

    // Assert
    expect(screen.queryByTestId('main-page')).not.toBeInTheDocument();
    expect(screen.getByTestId('entry-page')).toBeInTheDocument();
  });

  it('renders MainPage when user is authenticated', () => {
    // Arrange
    useAuth.mockReturnValue({ user: 'testuser' });

    // Act
    render(<RequireAuth />);

    // Assert
    expect(screen.getByTestId('main-page')).toBeInTheDocument();
    expect(screen.queryByTestId('entry-page')).not.toBeInTheDocument();
  });
});
