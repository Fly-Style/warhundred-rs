import {useEffect, useRef, useState} from "react";
import PropTypes from 'prop-types';
import * as PIXI from "pixi.js";
import '../GameWindow.css'
import {updateDimensions} from "../../../util/utils.js";

/**
 * BaseCanvas component for rendering PIXI.js canvas
 * @param {Object} props - Component props
 * @param {number} props.backgroundColor - Background color in hexadecimal format
 * @param {Function} props.renderContent - Function to render custom content on the canvas
 * @returns {JSX.Element} - Rendered component
 */
const BaseCanvas = ({ backgroundColor = 0x000000, renderContent }) => {
  const pixiContainer = useRef(null);
  const [dimensions, setDimensions] = useState({width: 0, height: 0});

  useEffect(() => {
    updateDimensions(pixiContainer, dimensions, setDimensions);

    const app = new PIXI.Application({
      height: dimensions.height,
      width: dimensions.width,
      autoResize: true,
      backgroundColor
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

    // Call the renderContent function to add custom graphics
    if (renderContent && typeof renderContent === 'function') {
      renderContent(app);
    }

    window.addEventListener("resize", canvasResizeObserver);

    return () => {
      window.removeEventListener("resize", canvasResizeObserver);
      app.destroy(true, true);
    } // Cleanup
  }, [pixiContainer, dimensions, backgroundColor, renderContent]);

  return <div ref={pixiContainer} className="game-window-canvas"/>;
};

BaseCanvas.propTypes = {
  backgroundColor: PropTypes.number,
  renderContent: PropTypes.func
};

export default BaseCanvas;
