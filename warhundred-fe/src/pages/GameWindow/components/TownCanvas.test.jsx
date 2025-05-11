import { describe, it, expect, vi } from 'vitest';
import { render } from '@testing-library/react';
import TownCanvas from './TownCanvas';
import BaseCanvas from './BaseCanvas';
import * as PIXI from 'pixi.js';

// Mock BaseCanvas
vi.mock('./BaseCanvas.jsx', () => ({
  default: vi.fn().mockImplementation(({ backgroundColor, renderContent }) => {
    // Store the renderContent function for testing
    if (renderContent) {
      BaseCanvas.mockRenderContent = renderContent;
    }
    return <div data-testid="base-canvas" data-background-color={backgroundColor} />;
  })
}));

// Mock PIXI.js
vi.mock('pixi.js', () => {
  const mockBeginFill = vi.fn();
  const mockDrawRect = vi.fn();
  const mockEndFill = vi.fn();
  const mockAddChild = vi.fn();

  return {
    Graphics: vi.fn().mockImplementation(() => ({
      beginFill: mockBeginFill,
      drawRect: mockDrawRect,
      endFill: mockEndFill
    })),
    mockBeginFill,
    mockDrawRect,
    mockEndFill,
    mockAddChild
  };
});

describe('TownCanvas', () => {
  it('renders a BaseCanvas with the correct props', () => {
    // Arrange & Act
    render(<TownCanvas />);

    // Assert
    expect(BaseCanvas).toHaveBeenCalledWith(
      expect.objectContaining({
        backgroundColor: 0x7CAFC2,
        renderContent: expect.any(Function)
      }),
      expect.anything()
    );
  });

  it('renderTownContent creates a green rectangle and adds it to the stage', () => {
    // Arrange
    render(<TownCanvas />);

    // Get the renderContent function that was passed to BaseCanvas
    const renderTownContent = BaseCanvas.mockRenderContent;

    // Mock PIXI.Application
    const mockApp = {
      stage: {
        addChild: PIXI.mockAddChild
      }
    };

    // Act
    renderTownContent(mockApp);

    // Assert
    // Verify that a Graphics object was created
    expect(PIXI.Graphics).toHaveBeenCalled();

    // Verify that beginFill was called with the correct color (green)
    expect(PIXI.mockBeginFill).toHaveBeenCalledWith(0x00ff00);

    // Verify that drawRect was called with the correct coordinates and dimensions
    expect(PIXI.mockDrawRect).toHaveBeenCalledWith(100, 100, 200, 200);

    // Verify that endFill was called
    expect(PIXI.mockEndFill).toHaveBeenCalled();

    // Verify that the graphics object was added to the stage
    expect(PIXI.mockAddChild).toHaveBeenCalled();
  });
});
