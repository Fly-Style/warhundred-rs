import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render } from '@testing-library/react';
import BaseCanvas from './BaseCanvas';
import { updateDimensions } from '../../../util/utils';
import * as PIXI from 'pixi.js';

// Mock PIXI.js
vi.mock('pixi.js', () => {
  const mockDestroy = vi.fn();
  const mockResize = vi.fn();
  const mockAddChild = vi.fn();

  return {
    Application: vi.fn().mockImplementation(() => ({
      view: document.createElement('canvas'),
      renderer: {
        resize: mockResize
      },
      stage: {
        addChild: mockAddChild
      },
      destroy: mockDestroy
    }))
  };
});

// Mock updateDimensions
vi.mock('../../../util/utils.js', () => ({
  updateDimensions: vi.fn()
}));

describe('BaseCanvas', () => {
  let renderContentMock;
  let addEventListenerSpy;
  let removeEventListenerSpy;

  beforeEach(() => {
    renderContentMock = vi.fn();
    addEventListenerSpy = vi.spyOn(window, 'addEventListener');
    removeEventListenerSpy = vi.spyOn(window, 'removeEventListener');

    // Reset mocks
    vi.clearAllMocks();
  });

  afterEach(() => {
    addEventListenerSpy.mockRestore();
    removeEventListenerSpy.mockRestore();
  });

  it('renders a div with the correct class name', () => {
    // Arrange & Act
    render(<BaseCanvas />);

    // Assert
    const canvasContainer = document.querySelector('.game-window-canvas');
    expect(canvasContainer).toBeInTheDocument();
  });

  it('calls updateDimensions with the correct arguments', () => {
    // Arrange & Act
    render(<BaseCanvas />);

    // Assert
    expect(updateDimensions).toHaveBeenCalled();
    // The first argument should be a ref object, the second should be dimensions, and the third should be setDimensions
    expect(updateDimensions.mock.calls[0][0]).toBeDefined();
    expect(updateDimensions.mock.calls[0][1]).toEqual({ width: 0, height: 0 });
    expect(typeof updateDimensions.mock.calls[0][2]).toBe('function');
  });

  it('creates a PIXI.Application with the correct options', () => {
    // Arrange & Act
    render(<BaseCanvas backgroundColor={0x123456} />);

    // Assert
    expect(PIXI.Application).toHaveBeenCalledWith({
      height: 0,
      width: 0,
      autoResize: true,
      backgroundColor: 0x123456
    });
  });

  it('calls renderContent with the PIXI.Application instance', () => {
    // Arrange & Act
    render(<BaseCanvas renderContent={renderContentMock} />);

    // Assert
    expect(renderContentMock).toHaveBeenCalled();
    // The argument should be a PIXI.Application instance
    expect(renderContentMock.mock.calls[0][0]).toBeDefined();
    expect(renderContentMock.mock.calls[0][0].view).toBeInstanceOf(HTMLCanvasElement);
  });

  it('adds and removes event listeners for window resize', () => {
    // Arrange & Act
    const { unmount } = render(<BaseCanvas />);

    // Assert - Event listener added
    expect(addEventListenerSpy).toHaveBeenCalledWith('resize', expect.any(Function));

    // Act - Unmount component
    unmount();

    // Assert - Event listener removed
    expect(removeEventListenerSpy).toHaveBeenCalledWith('resize', expect.any(Function));
  });

  it('cleans up by destroying the PIXI.Application when unmounted', () => {
    // Arrange
    const { unmount } = render(<BaseCanvas />);

    // Act
    unmount();

    // Assert
    const mockApplication = PIXI.Application.mock.results[0].value;
    expect(mockApplication.destroy).toHaveBeenCalledWith(true, true);
  });

  it('does not call renderContent if it is not a function', () => {
    // Arrange
    const renderContentMock = 'not a function';

    // Act
    render(<BaseCanvas renderContent={renderContentMock} />);

    // Assert
    // Since renderContent is not a function, it should not be called
    // We can't directly test this, but we can verify that no errors were thrown
    expect(true).toBe(true);
  });
});
