import {useState} from "react";
import {useNavigate} from "react-router-dom";
import axios from "axios";
import "../EntryPage.css"
import {handleFormChange} from "../../../util/utils.js";

export const RegisterForm = () => {
  const [formData, setFormData] = useState({username: "", email: "", password: ""});
  const nav = useNavigate();


  const submitHandler = () => {
    event.preventDefault();
    axios.post(`${import.meta.env.VITE_SERVER_URL}/register`, formData, {
      headers: {'Content-Type': 'application/json',}
    }).then(res => {
      alert("Register is successful.")
    }).catch(err => console.log(err));
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
            required
          />
        </label>
        <label>
          E-mail:
          <input
            type="email"
            name="email"
            value={formData.email}
            className="entry-page__input"
            onChange={(e) => handleFormChange(e, setFormData)}
            required
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
            required
          />
        </label>
        <input
          type="submit"
          value="Register"
          className="entry-page__submit"
        />
      </form>
    </>
  )
}