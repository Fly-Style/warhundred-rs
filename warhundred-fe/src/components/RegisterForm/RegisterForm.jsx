import {useState} from "react";
import axios from "axios";
import "./RegisterForm.css"

export const RegisterForm = () => {
  const [formData, setFormData] = useState({
    username: "",
    email: "",
    password: ""
  });

  const handleChange = (event) => {
    const {name, value} = event.target;
    setFormData((prevState) => ({...prevState, [name]: value}));
  };

  const submitHandler = () => {
    event.preventDefault();
    axios.post("/register", formData)
      .then(res => alert(res))
      .catch(err => console.log(event));
  }

  return (
    <>
      <form onSubmit={submitHandler} className="main">
        <label>
          Username:
          <input
            type="text"
            name="username"
            value={formData.username}
            onChange={handleChange}
          />
        </label>
        <label>
          E-mail:
          <input
            type="email"
            name="email"
            value={formData.email}
            onChange={handleChange}
          />
        </label>
        <label>
          Password:
          <input
            type="password"
            name="password"
            value={formData.password}
            onChange={handleChange}
          />
        </label>
        <input type="submit" className="submit" value="Submit"/>
      </form>
    </>
  )
}