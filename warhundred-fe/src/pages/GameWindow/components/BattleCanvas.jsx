import {useEffect, useRef, useState} from "react";
import * as PIXI from "pixi.js";
import '../GameWindow.css'
import {updateDimensions} from "../../../util/utils.js";

const BattleCanvas = () => {
  const pixiContainer = useRef(null);
  const [dimensions, setDimensions] = useState({width: 0, height: 0});

  useEffect(() => {
    updateDimensions(pixiContainer, dimensions, setDimensions);

    const app = new PIXI.Application({
      height: dimensions.height,
      width: dimensions.width,
      autoResize: true,
      backgroundColor: 0x000000
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
    building.beginFill(0xffff00);
    building.drawRect(50, 150, 200, 300); // A green "building"
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

export default BattleCanvas;