import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import EntryPage from './EntryPage';

// Mock the form components
vi.mock('./components/RegisterForm.jsx', () => ({
  RegisterForm: () => <div data-testid="register-form">Register Form</div>
}));

vi.mock('./components/LoginForm.jsx', () => ({
  LoginForm: () => <div data-testid="login-form">Login Form</div>
}));

describe('EntryPage', () => {
  it('renders correctly with welcome message and tabs', () => {
    // Arrange & Act
    render(<EntryPage />);

    // Assert
    expect(screen.getByText('Welcome to War Hundred')).toBeInTheDocument();
    expect(screen.getByText('Register')).toBeInTheDocument();
    expect(screen.getByText('Login')).toBeInTheDocument();
    expect(screen.getByText('All rights reserved (c)')).toBeInTheDocument();
  });

  it('renders RegisterForm in the first tab by default', () => {
    // Arrange & Act
    render(<EntryPage />);

    // Assert
    expect(screen.getByTestId('register-form')).toBeInTheDocument();
    expect(screen.queryByTestId('login-form')).not.toBeInTheDocument();
  });

  it('renders LoginForm when Login tab is clicked', () => {
    // Arrange
    render(<EntryPage />);

    // Act - Click the Login tab
    const loginTab = screen.getByText('Login');
    fireEvent.click(loginTab);

    // Assert
    expect(screen.queryByTestId('register-form')).not.toBeInTheDocument();
    expect(screen.getByTestId('login-form')).toBeInTheDocument();
  });

  it('allows switching between tabs', () => {
    // Arrange
    render(<EntryPage />);

    // Initially shows RegisterForm
    expect(screen.getByTestId('register-form')).toBeInTheDocument();
    expect(screen.queryByTestId('login-form')).not.toBeInTheDocument();

    // Act - Switch to Login tab
    const loginTab = screen.getByText('Login');
    fireEvent.click(loginTab);

    // Assert - Now shows LoginForm
    expect(screen.queryByTestId('register-form')).not.toBeInTheDocument();
    expect(screen.getByTestId('login-form')).toBeInTheDocument();

    // Act - Switch back to Register tab
    const registerTab = screen.getByText('Register');
    fireEvent.click(registerTab);

    // Assert - Now shows RegisterForm again
    expect(screen.getByTestId('register-form')).toBeInTheDocument();
    expect(screen.queryByTestId('login-form')).not.toBeInTheDocument();
  });

  it('has the correct CSS classes for styling', () => {
    // Arrange & Act
    render(<EntryPage />);

    // Assert
    const main = screen.getByRole('main');
    expect(main).toHaveClass('entry-page__main');

    const tabList = screen.getByRole('tablist');
    expect(tabList).toHaveClass('entry-page__tab-list');

    const tabs = screen.getAllByRole('tab');
    tabs.forEach(tab => {
      expect(tab).toHaveClass('entry-page__list-item');
    });

    const buttons = screen.getAllByRole('button', { name: /register|login/i });
    buttons.forEach(button => {
      expect(button).toHaveClass('entry-page__tab-list-btn');
    });
  });
});
