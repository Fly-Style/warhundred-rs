import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import ErrorBoundary from './ErrorBoundary';
import React from 'react';
import PropTypes from 'prop-types';

// Component that throws an error when the shouldThrow prop is true
const ErrorThrowingComponent = ({ shouldThrow }) => {
  if (shouldThrow) {
    throw new Error('Test error');
  }
  return <div data-testid="normal-component">Normal component</div>;
};

ErrorThrowingComponent.propTypes = {
  shouldThrow: PropTypes.bool
};

// Suppress console.error during tests to avoid noisy output
const originalConsoleError = console.error;
beforeEach(() => {
  console.error = vi.fn();
});

afterEach(() => {
  console.error = originalConsoleError;
});

describe('ErrorBoundary', () => {
  it('renders children correctly when there is no error', () => {
    // Arrange & Act
    render(
      <ErrorBoundary>
        <div data-testid="child">Child content</div>
      </ErrorBoundary>
    );

    // Assert
    const child = screen.getByTestId('child');
    expect(child).toBeInTheDocument();
    expect(child).toHaveTextContent('Child content');
  });

  it('renders the default error UI when an error occurs', () => {
    // Arrange & Act
    // We need to suppress the error boundary warning in the console
    const spy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(
      <ErrorBoundary>
        <ErrorThrowingComponent shouldThrow={true} />
      </ErrorBoundary>
    );

    // Assert
    expect(screen.getByText('Something went wrong.')).toBeInTheDocument();
    expect(screen.getByText('Error details')).toBeInTheDocument();
    expect(screen.getByText('Try again')).toBeInTheDocument();

    // Cleanup
    spy.mockRestore();
  });

  it('renders a custom fallback component when provided and an error occurs', () => {
    // Arrange
    const fallback = ({ error, resetErrorBoundary }) => (
      <div>
        <h2 data-testid="custom-fallback">Custom error: {error.message}</h2>
        <button onClick={resetErrorBoundary}>Reset</button>
      </div>
    );

    // Act
    // We need to suppress the error boundary warning in the console
    const spy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(
      <ErrorBoundary fallback={fallback}>
        <ErrorThrowingComponent shouldThrow={true} />
      </ErrorBoundary>
    );

    // Assert
    expect(screen.getByTestId('custom-fallback')).toBeInTheDocument();
    expect(screen.getByText('Custom error: Test error')).toBeInTheDocument();

    // Cleanup
    spy.mockRestore();
  });

  it('calls onError when an error occurs', () => {
    // Arrange
    const onError = vi.fn();

    // Act
    // We need to suppress the error boundary warning in the console
    const spy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(
      <ErrorBoundary onError={onError}>
        <ErrorThrowingComponent shouldThrow={true} />
      </ErrorBoundary>
    );

    // Assert
    expect(onError).toHaveBeenCalledTimes(1);
    expect(onError.mock.calls[0][0]).toBeInstanceOf(Error);
    expect(onError.mock.calls[0][0].message).toBe('Test error');

    // Cleanup
    spy.mockRestore();
  });

  it('resets error state when the "Try again" button is clicked', () => {
    // Arrange
    const TestComponent = () => {
      const [shouldThrow, setShouldThrow] = React.useState(true);

      return (
        <ErrorBoundary onReset={() => setShouldThrow(false)}>
          <ErrorThrowingComponent shouldThrow={shouldThrow} />
        </ErrorBoundary>
      );
    };

    // Act
    // We need to suppress the error boundary warning in the console
    const spy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(<TestComponent />);

    // Assert - Initially shows error
    expect(screen.getByText('Something went wrong.')).toBeInTheDocument();

    // Act - Click the "Try again" button
    fireEvent.click(screen.getByText('Try again'));

    // Assert - Now shows the normal component
    expect(screen.getByTestId('normal-component')).toBeInTheDocument();

    // Cleanup
    spy.mockRestore();
  });

  it('calls onReset when the "Try again" button is clicked', () => {
    // Arrange
    const onReset = vi.fn();

    // Act
    // We need to suppress the error boundary warning in the console
    const spy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(
      <ErrorBoundary onReset={onReset}>
        <ErrorThrowingComponent shouldThrow={true} />
      </ErrorBoundary>
    );

    // Assert - Initially shows error
    expect(screen.getByText('Something went wrong.')).toBeInTheDocument();

    // Act - Click the "Try again" button
    fireEvent.click(screen.getByText('Try again'));

    // Assert - onReset was called
    expect(onReset).toHaveBeenCalledTimes(1);

    // Cleanup
    spy.mockRestore();
  });
});
