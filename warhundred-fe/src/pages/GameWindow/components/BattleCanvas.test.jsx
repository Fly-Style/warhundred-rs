import { describe, it, expect, vi } from 'vitest';
import { render } from '@testing-library/react';
import BattleCanvas from './BattleCanvas';
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

describe('BattleCanvas', () => {
  it('renders a BaseCanvas with the correct props', () => {
    // Arrange & Act
    render(<BattleCanvas />);

    // Assert
    expect(BaseCanvas).toHaveBeenCalledWith(
      expect.objectContaining({
        backgroundColor: 0x000000,
        renderContent: expect.any(Function)
      }),
      expect.anything()
    );
  });

  it('renderBattleContent creates a yellow rectangle and adds it to the stage', () => {
    // Arrange
    render(<BattleCanvas />);

    // Get the renderContent function that was passed to BaseCanvas
    const renderBattleContent = BaseCanvas.mockRenderContent;

    // Mock PIXI.Application
    const mockApp = {
      stage: {
        addChild: PIXI.mockAddChild
      }
    };

    // Act
    renderBattleContent(mockApp);

    // Assert
    // Verify that a Graphics object was created
    expect(PIXI.Graphics).toHaveBeenCalled();

    // Verify that beginFill was called with the correct color (yellow)
    expect(PIXI.mockBeginFill).toHaveBeenCalledWith(0xffff00);

    // Verify that drawRect was called with the correct coordinates and dimensions
    expect(PIXI.mockDrawRect).toHaveBeenCalledWith(50, 150, 200, 300);

    // Verify that endFill was called
    expect(PIXI.mockEndFill).toHaveBeenCalled();

    // Verify that the graphics object was added to the stage
    expect(PIXI.mockAddChild).toHaveBeenCalled();
  });
});
