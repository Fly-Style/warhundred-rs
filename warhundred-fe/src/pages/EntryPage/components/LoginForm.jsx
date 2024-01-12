import {useState} from "react";
import axios from "axios";
import "../EntryPage.css"
import {handleFormChange} from "../../../util/utils.js";

export const LoginForm = () => {
  const [formData, setFormData] = useState({
    username: "",
    password: ""
  });

  const submitHandler = () => {
    event.preventDefault();
    axios.post("/login", formData, {
      headers: {'Content-Type': 'application/json',}
    })
      .then(res => alert(res))
      .catch(err => console.log(err));
  }

  return (
    <>
      <form onSubmit={submitHandler} className="entry-page__form">
        <label>
          Username:
          <input
            type="text"
            name="username"
            value={formData.username}
            className="entry-page__input"
            onChange={(e) => handleFormChange(e, setFormData)}
          />
        </label>
        <label>
          Password:
          <input
            type="password"
            name="password"
            value={formData.password}
            className="entry-page__input"
            onChange={(e) => handleFormChange(e, setFormData)}
          />
        </label>
        <input
          type="submit"
          value="Login"
          className="entry-page__submit"
        />
      </form>
    </>
  )
}