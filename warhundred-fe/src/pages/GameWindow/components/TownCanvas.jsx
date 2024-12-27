import React, {useEffect, useRef, useState} from "react";
import * as PIXI from "pixi.js";
import '../GameWindow.css'

const TownCanvas = () => {
  const pixiContainer = useRef(null);
  const [dimensions, setDimensions] = useState({ width: 0, height: 0 });

  useEffect(() => {
    if (pixiContainer.current) {
      setDimensions( {
          width: pixiContainer.current.offsetWidth,
          height: pixiContainer.current.offsetHeight
      })
    }

    const app = new PIXI.Application({
      height: dimensions.height,
      width: dimensions.width,
      resizeTo: pixiContainer.current,
      backgroundColor: 0x87ceeb
    });
    pixiContainer.current.appendChild(app.view);

    // Add static town graphics (like buildings or trees)
    const building = new PIXI.Graphics();
    building.beginFill(0x00ff00);
    building.drawRect(50, 150, 200, 300); // A green "building"
    building.endFill();
    app.stage.addChild(building);

    window.addEventListener("resize", () => {
      if (pixiContainer.current) {
        app.resizeTo(pixiContainer.current)
      }
    });

    return () => app.destroy(true, true); // Cleanup
  }, [pixiContainer]);

  return <div ref={pixiContainer} className="dumb-fuck"/>;
};

export default TownCanvas;