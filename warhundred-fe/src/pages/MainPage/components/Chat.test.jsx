import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Chat } from './Chat';

describe('Chat', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset timers
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('renders with test data when useTestData is true', () => {
    // Arrange & Act
    render(<Chat useTestData={true} />);

    // Assert
    expect(screen.getByText('Chat')).toBeInTheDocument();
    expect(screen.getByText('5 messages')).toBeInTheDocument();

    // Check if all test messages are rendered
    expect(screen.getByText('Welcome to the chat!')).toBeInTheDocument();
    expect(screen.getByText('Hello everyone!')).toBeInTheDocument();
    expect(screen.getByText('Hi there!')).toBeInTheDocument();
    expect(screen.getByText('Anyone want to team up for a raid?')).toBeInTheDocument();
    expect(screen.getByText('I can join as a healer')).toBeInTheDocument();

    // Check for message senders
    expect(screen.getByText('System')).toBeInTheDocument();
    expect(screen.getByText('Warrior123')).toBeInTheDocument();
    expect(screen.getByText('ArcherQueen')).toBeInTheDocument();
    expect(screen.getByText('DarkWizard')).toBeInTheDocument();
    expect(screen.getByText('HealerGirl')).toBeInTheDocument();

    // Check for timestamps
    expect(screen.getByText('10:00')).toBeInTheDocument();
    expect(screen.getByText('10:05')).toBeInTheDocument();
    expect(screen.getByText('10:07')).toBeInTheDocument();
    expect(screen.getByText('10:10')).toBeInTheDocument();
    expect(screen.getByText('10:12')).toBeInTheDocument();
  });

  it('shows loading state when fetching data', () => {
    // Arrange & Act
    render(<Chat />);

    // Assert
    expect(screen.getByText('Loading messages...')).toBeInTheDocument();
  });

  it('allows sending a new message', () => {
    // Arrange
    render(<Chat useTestData={true} />);

    // Act - Type a message and send it
    const input = screen.getByPlaceholderText('Type your message...');
    fireEvent.change(input, { target: { value: 'Hello, world!' } });

    const sendButton = screen.getByText('Send');
    fireEvent.click(sendButton);

    // Assert
    expect(screen.getByText('Hello, world!')).toBeInTheDocument();
    expect(screen.getByText('You')).toBeInTheDocument();

    // Check that the input was cleared
    expect(input.value).toBe('');

    // Check that the message count was updated
    expect(screen.getByText('6 messages')).toBeInTheDocument();
  });

  it('allows sending a message with Enter key', () => {
    // Arrange
    render(<Chat useTestData={true} />);

    // Act - Type a message and press Enter
    const input = screen.getByPlaceholderText('Type your message...');
    fireEvent.change(input, { target: { value: 'Hello from keyboard!' } });
    fireEvent.keyDown(input, { key: 'Enter', code: 'Enter' });

    // Assert
    expect(screen.getByText('Hello from keyboard!')).toBeInTheDocument();
    expect(input.value).toBe('');
  });

  it('does not send empty messages', () => {
    // Arrange
    render(<Chat useTestData={true} />);
    const initialMessageCount = screen.getByText('5 messages');

    // Act - Try to send an empty message
    const sendButton = screen.getByText('Send');
    fireEvent.click(sendButton);

    // Assert - Message count should still be 5
    expect(initialMessageCount).toBeInTheDocument();
  });

  it('disables the send button when the input is empty', () => {
    // Arrange
    render(<Chat useTestData={true} />);

    // Act & Assert
    const sendButton = screen.getByText('Send');
    expect(sendButton).toBeDisabled();

    // Type something and check that the button is enabled
    const input = screen.getByPlaceholderText('Type your message...');
    fireEvent.change(input, { target: { value: 'Test message' } });
    expect(sendButton).not.toBeDisabled();

    // Clear the input and check that the button is disabled again
    fireEvent.change(input, { target: { value: '' } });
    expect(sendButton).toBeDisabled();
  });

  it('cleans up interval on unmount', () => {
    // Arrange
    const clearIntervalSpy = vi.spyOn(window, 'clearInterval');

    // Use useTestData={false} to ensure the interval is set up
    const { unmount } = render(<Chat useTestData={false} />);

    // Act
    unmount();

    // Assert
    expect(clearIntervalSpy).toHaveBeenCalled();

    // Clean up
    clearIntervalSpy.mockRestore();
  });
});
