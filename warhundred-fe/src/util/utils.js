export const handleFormChange = (event, formDataSetter) => {
  const {name, value} = event.target;
  formDataSetter((prevState) => ({...prevState, [name]: value}));
}

export const updateDimensions = (pixiContainer, dimensions, setDimensions) => {
  if (pixiContainer.current &&
      (dimensions.width !== pixiContainer.current.offsetWidth
          || dimensions.height !== pixiContainer.current.offsetHeight)) {
    setDimensions({
      width: pixiContainer.current.offsetWidth,
      height: pixiContainer.current.offsetHeight
    })
  }
}