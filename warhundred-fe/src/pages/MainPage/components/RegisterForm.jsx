import {useState} from "react";
import axios from "axios";
import "../MainPage.css"
import {handleFormChange} from "../../../util/utils.js";

export const RegisterForm = () => {
  const [formData, setFormData] = useState({
    username: "",
    email: "",
    password: ""
  });

  const submitHandler = () => {
    event.preventDefault();
    console.log(formData);
    axios.post("/register", formData, {
      headers: {'Content-Type': 'application/json',}
    })
      .then(res => alert(res))
      .catch(err => console.log(err));
  }

  return (
    <>
      <form onSubmit={submitHandler} className="main-page__form">
        <label>
          Username:
          <input
            type="text"
            name="username"
            value={formData.username}
            className="main-page__input"
            onChange={(e) => handleFormChange(e, setFormData)}
          />
        </label>
        <label>
          E-mail:
          <input
            type="email"
            name="email"
            value={formData.email}
            className="main-page__input"
            onChange={(e) => handleFormChange(e, setFormData)}
          />
        </label>
        <label>
          Password:
          <input
            type="password"
            name="password"
            value={formData.password}
            className="main-page__input"
            onChange={(e) => handleFormChange(e, setFormData)}
          />
        </label>
        <input
          type="submit"
          value="Register"
          className="main-page__submit"
        />
      </form>
    </>
  )
}