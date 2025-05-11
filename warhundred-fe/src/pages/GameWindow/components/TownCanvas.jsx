import * as PIXI from "pixi.js";
import '../GameWindow.css'
import BaseCanvas from "./BaseCanvas.jsx";

/**
 * TownCanvas component for rendering the town view
 * @returns {JSX.Element} - Rendered component
 */
const TownCanvas = () => {
  /**
   * Render town-specific content on the canvas
   * @param {PIXI.Application} app - The PIXI Application instance
   */
  const renderTownContent = (app) => {
    // Add static town graphics (like buildings or trees)
    const building = new PIXI.Graphics();
    building.beginFill(0x00ff00);
    building.drawRect(100, 100, 200, 200);
    building.endFill();
    app.stage.addChild(building);
  };

  return <BaseCanvas backgroundColor={0x87ceeb} renderContent={renderTownContent} />;
};

export default TownCanvas;
