import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import Input from './Input';
import React from 'react';

describe('Input', () => {
  it('renders correctly with default props', () => {
    // Arrange & Act
    render(<Input name="test" />);

    // Assert
    const input = screen.getByRole('textbox');
    expect(input).toBeInTheDocument();
    expect(input).toHaveAttribute('name', 'test');
    expect(input).toHaveAttribute('id', 'test');
    expect(input).toHaveAttribute('type', 'text');
    expect(input).toHaveClass('ui-input');
  });

  it('applies the correct CSS classes based on props', () => {
    // Arrange & Act
    render(
      <Input 
        name="test" 
        error="Error message" 
        className="custom-class"
      />
    );

    // Assert
    const input = screen.getByRole('textbox');
    expect(input).toHaveClass('ui-input');
    expect(input).toHaveClass('ui-input--error');
    expect(input).toHaveClass('custom-class');
  });

  it('passes inputProps to the input element', () => {
    // Arrange
    const handleChange = vi.fn();

    // Act
    render(
      <Input 
        name="test" 
        inputProps={{ 
          onChange: handleChange,
          placeholder: 'Enter text',
          required: true,
          disabled: false,
          'data-testid': 'test-input'
        }}
      />
    );

    // Assert
    const input = screen.getByTestId('test-input');
    expect(input).toHaveAttribute('placeholder', 'Enter text');
    expect(input).toHaveAttribute('required');
    expect(input).not.toBeDisabled();

    // Type in the input
    fireEvent.change(input, { target: { value: 'Hello' } });
    expect(handleChange).toHaveBeenCalledTimes(1);
  });

  it('renders label correctly', () => {
    // Arrange & Act
    render(
      <Input 
        name="test" 
        label="Test Label" 
        inputProps={{ required: true }}
      />
    );

    // Assert
    const label = screen.getByText('Test Label');
    expect(label).toBeInTheDocument();

    // Check for required indicator
    const requiredIndicator = screen.getByText('*');
    expect(requiredIndicator).toBeInTheDocument();
    expect(requiredIndicator).toHaveClass('ui-input-required');
  });

  it('renders error message correctly', () => {
    // Arrange & Act
    render(
      <Input 
        name="test" 
        error="Error message"
      />
    );

    // Assert
    const errorMessage = screen.getByText('Error message');
    expect(errorMessage).toBeInTheDocument();
    expect(errorMessage).toHaveClass('ui-input-error');

    // Check aria-invalid attribute
    const input = screen.getByRole('textbox');
    expect(input).toHaveAttribute('aria-invalid', 'true');
  });

  it('forwards ref correctly', () => {
    // Arrange
    const ref = React.createRef();

    // Act
    render(<Input name="test" ref={ref} />);

    // Assert
    expect(ref.current).toBeInstanceOf(HTMLInputElement);
    expect(ref.current.name).toBe('test');
  });
});
