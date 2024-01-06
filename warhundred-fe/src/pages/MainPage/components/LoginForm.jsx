import {useState} from "react";
import axios from "axios";
import "../MainPage.css"
import {handleFormChange} from "../../../util/utils.js";

export const LoginForm = () => {
  const [formData, setFormData] = useState({
    username: "",
    password: ""
  });

  const submitHandler = () => {
    event.preventDefault();
    axios.post("/login", formData)
      .then(res => alert(res))
      .catch(err => console.log(event));
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
          value="Login"
          className="main-page__submit"
        />
      </form>
    </>
  )
}