export const handleFormChange = (event, formDataSetter) => {
  const {name, value} = event.target;
  formDataSetter((prevState) => ({...prevState, [name]: value}));
}