import {useEffect, useRef, useState} from "react";
import * as PIXI from "pixi.js";
import '../GameWindow.css'
import {updateDimensions} from "../../../util/utils.js";

const TownCanvas = () => {
  const pixiContainer = useRef(null);
  const [dimensions, setDimensions] = useState({width: 0, height: 0});

  useEffect(() => {
    updateDimensions(pixiContainer, dimensions, setDimensions);

    const app = new PIXI.Application({
      height: dimensions.height,
      width: dimensions.width,
      autoResize: true,
      backgroundColor: 0x87ceeb
    });
    pixiContainer.current.appendChild(app.view);

    const canvasResizeObserver = () => {
      if (pixiContainer.current &&
          (dimensions.width !== pixiContainer.current.offsetWidth
              || dimensions.height !== pixiContainer.current.offsetHeight)) {
        setDimensions({
          width: pixiContainer.current.offsetWidth,
          height: pixiContainer.current.offsetHeight
        })
      }
      app.renderer.resize(dimensions.width, dimensions.height);
    }

    // Add static town graphics (like buildings or trees)
    const building = new PIXI.Graphics();
    building.beginFill(0x00ff00);
    building.drawRect(100, 100, 200, 200);
    building.endFill();
    app.stage.addChild(building);

    window.addEventListener("resize", canvasResizeObserver);

    return () => {
      window.removeEventListener("resize", canvasResizeObserver);
      app.destroy(true, true);
    } // Cleanup
  }, [pixiContainer, dimensions]);

  return <div ref={pixiContainer} className="game-window-canvas"/>;
};

export default TownCanvas;