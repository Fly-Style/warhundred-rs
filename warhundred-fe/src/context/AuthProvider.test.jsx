import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, waitFor, fireEvent } from '@testing-library/react';
import { AuthProvider, useAuth } from './AuthProvider';

// Mock the authService
vi.mock('../services/authService', () => ({
  default: {
    login: vi.fn(),
    logout: vi.fn(),
    getToken: vi.fn(),
    isAuthenticated: vi.fn()
  }
}));

// Import the mocked service
import authService from '../services/authService';

// Test component that uses the auth context
const TestComponent = () => {
  const auth = useAuth();
  return (
    <div>
      <div data-testid="user">{auth.user || 'No user'}</div>
      <div data-testid="loading">{auth.isLoading.toString()}</div>
      <div data-testid="errors">{auth.errors.toString()}</div>
      <button onClick={() => auth.login('testuser', 'password')}>Login</button>
      <button onClick={() => auth.logout()}>Logout</button>
    </div>
  );
};

describe('AuthProvider', () => {
  beforeEach(() => {
    // Clear all mocks before each test
    vi.clearAllMocks();
  });

  afterEach(() => {
    // Reset localStorage mock after each test
    localStorage.clear();
  });

  it('renders children correctly', () => {
    // Arrange & Act
    render(
      <AuthProvider>
        <div data-testid="child">Child content</div>
      </AuthProvider>
    );

    // Assert
    const child = screen.getByTestId('child');
    expect(child).toBeInTheDocument();
    expect(child).toHaveTextContent('Child content');
  });

  it('provides initial auth context values', () => {
    // Arrange & Act
    render(
      <AuthProvider>
        <TestComponent />
      </AuthProvider>
    );

    // Assert
    expect(screen.getByTestId('user')).toHaveTextContent('No user');
    expect(screen.getByTestId('loading')).toHaveTextContent('false');
    expect(screen.getByTestId('errors')).toHaveTextContent('');
  });

  it('calls login function and updates state on success', async () => {
    // Arrange
    authService.login.mockResolvedValueOnce({
      data: { nickname: 'testuser' }
    });

    // Act
    render(
      <AuthProvider>
        <TestComponent />
      </AuthProvider>
    );

    const loginButton = screen.getByText('Login');
    fireEvent.click(loginButton);

    // Assert
    expect(authService.login).toHaveBeenCalledWith('testuser', 'password');

    // Wait for state updates
    await waitFor(() => {
      expect(screen.getByTestId('user')).toHaveTextContent('testuser');
      expect(screen.getByTestId('loading')).toHaveTextContent('false');
    });
  });

  it('handles login errors correctly', async () => {
    // Arrange
    const error = new Error('Login failed');
    authService.login.mockRejectedValueOnce(error);

    // Act
    render(
      <AuthProvider>
        <TestComponent />
      </AuthProvider>
    );

    const loginButton = screen.getByText('Login');
    fireEvent.click(loginButton);

    // Assert
    expect(authService.login).toHaveBeenCalledWith('testuser', 'password');

    // Wait for state updates
    await waitFor(() => {
      expect(screen.getByTestId('user')).toHaveTextContent('No user');
      expect(screen.getByTestId('loading')).toHaveTextContent('false');
      expect(screen.getByTestId('errors')).toHaveTextContent('Error: Login failed');
    });
  });

  it('calls logout function and updates state on success', async () => {
    // Arrange
    authService.logout.mockResolvedValueOnce({});

    // Set initial state with a logged-in user
    authService.login.mockResolvedValueOnce({
      data: { nickname: 'testuser' }
    });

    // Act
    render(
      <AuthProvider>
        <TestComponent />
      </AuthProvider>
    );

    // First login
    const loginButton = screen.getByText('Login');
    fireEvent.click(loginButton);

    // Wait for login to complete
    await waitFor(() => {
      expect(screen.getByTestId('user')).toHaveTextContent('testuser');
    });

    // Then logout
    const logoutButton = screen.getByText('Logout');
    fireEvent.click(logoutButton);

    // Assert
    expect(authService.logout).toHaveBeenCalledWith('testuser');

    // Wait for state updates
    await waitFor(() => {
      expect(screen.getByTestId('user')).toHaveTextContent('No user');
      expect(screen.getByTestId('loading')).toHaveTextContent('false');
    });
  });

  it('handles logout errors correctly', async () => {
    // Arrange
    const error = new Error('Logout failed');

    // Set initial state with a logged-in user
    authService.login.mockResolvedValueOnce({
      data: { nickname: 'testuser' }
    });

    // Act
    render(
      <AuthProvider>
        <TestComponent />
      </AuthProvider>
    );

    // First login
    const loginButton = screen.getByText('Login');
    fireEvent.click(loginButton);

    // Wait for login to complete
    await waitFor(() => {
      expect(screen.getByTestId('user')).toHaveTextContent('testuser');
    });

    // Mock logout to fail
    authService.logout.mockRejectedValueOnce(error);

    // Then logout
    const logoutButton = screen.getByText('Logout');
    fireEvent.click(logoutButton);

    // Assert
    expect(authService.logout).toHaveBeenCalledWith('testuser');

    // Wait for state updates
    await waitFor(() => {
      expect(screen.getByTestId('user')).toHaveTextContent('testuser'); // User should still be logged in
      expect(screen.getByTestId('loading')).toHaveTextContent('false');
      expect(screen.getByTestId('errors')).toHaveTextContent('Error: Logout failed');
    });
  });
});
