import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import Button from './Button';

describe('Button', () => {
  it('renders correctly with default props', () => {
    // Arrange & Act
    render(<Button>Click me</Button>);

    // Assert
    const button = screen.getByRole('button', { name: /click me/i });
    expect(button).toBeInTheDocument();
    expect(button).toHaveClass('ui-button');
    expect(button).toHaveClass('ui-button--primary');
    expect(button).toHaveClass('ui-button--medium');
  });

  it('applies the correct CSS classes based on props', () => {
    // Arrange & Act
    render(
      <Button 
        variant="secondary" 
        size="large" 
        fullWidth 
        className="custom-class"
      >
        Click me
      </Button>
    );

    // Assert
    const button = screen.getByRole('button', { name: /click me/i });
    expect(button).toHaveClass('ui-button');
    expect(button).toHaveClass('ui-button--secondary');
    expect(button).toHaveClass('ui-button--large');
    expect(button).toHaveClass('ui-button--full-width');
    expect(button).toHaveClass('custom-class');
  });

  it('passes buttonProps to the button element and respects disabled state', () => {
    // Arrange
    const handleClick = vi.fn();

    // Act
    render(
      <Button 
        buttonProps={{ 
          onClick: handleClick,
          disabled: true,
          'data-testid': 'test-button'
        }}
      >
        Click me
      </Button>
    );

    // Assert
    const button = screen.getByTestId('test-button');
    expect(button).toBeDisabled();

    // Try to click the button
    fireEvent.click(button);
    expect(handleClick).not.toHaveBeenCalled();
  });

  it('calls onClick handler when button is clicked', () => {
    // Arrange
    const handleClick = vi.fn();

    // Act - render an enabled button
    render(
      <Button 
        buttonProps={{ 
          onClick: handleClick,
          'data-testid': 'enabled-button'
        }}
      >
        Click me
      </Button>
    );

    // Assert
    const button = screen.getByTestId('enabled-button');
    expect(button).not.toBeDisabled();

    // Click the button
    fireEvent.click(button);
    expect(handleClick).toHaveBeenCalledTimes(1);
  });

  it('renders children correctly', () => {
    // Arrange & Act
    render(
      <Button>
        <span data-testid="child">Child content</span>
      </Button>
    );

    // Assert
    const child = screen.getByTestId('child');
    expect(child).toBeInTheDocument();
    expect(child).toHaveTextContent('Child content');
  });
});
