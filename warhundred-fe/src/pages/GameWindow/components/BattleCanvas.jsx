import * as PIXI from "pixi.js";
import '../GameWindow.css'
import BaseCanvas from "./BaseCanvas.jsx";

/**
 * BattleCanvas component for rendering the battle view
 * @returns {JSX.Element} - Rendered component
 */
const BattleCanvas = () => {
  /**
   * Render battle-specific content on the canvas
   * @param {PIXI.Application} app - The PIXI Application instance
   */
  const renderBattleContent = (app) => {
    // Add battle-specific graphics
    const building = new PIXI.Graphics();
    building.beginFill(0xffff00);
    building.drawRect(50, 150, 200, 300);
    building.endFill();
    app.stage.addChild(building);
  };

  return <BaseCanvas backgroundColor={0x000000} renderContent={renderBattleContent} />;
};

export default BattleCanvas;
