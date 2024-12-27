import React, { useEffect, useRef } from "react";
import * as PIXI from "pixi.js";
import '../GameWindow.css'

const BattleCanvas = () => {
  const pixiContainer = useRef(null);

  useEffect(() => {
    const app = new PIXI.Application({
      // height: pixiContainer.current.getBoundingClientRect().height,
      // width: pixiContainer.current.getBoundingClientRect().width,
      autoResize: true,
      backgroundColor: 0x1e1e1e
    });
    pixiContainer.current.appendChild(app.view);

    // Add battle-specific graphics (like red battlefields)
    const battlefield = new PIXI.Graphics();
    battlefield.beginFill(0xff0000);
    battlefield.drawRect(50, 50, 400, 400);
    battlefield.endFill();
    app.stage.addChild(battlefield);

    return () => app.destroy(true, true); // Cleanup
  }, []);

  return <div ref={pixiContainer} className="dumb-fuck"/>;
};

export default BattleCanvas;