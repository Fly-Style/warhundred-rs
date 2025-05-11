import {useState} from "react";
// import {useNavigate} from "react-router-dom"; // Uncomment when navigation is needed
import axios from "axios";
import "../EntryPage.css"

export const RegisterForm = () => {
  // const nav = useNavigate(); // Uncomment when navigation is needed
  const [username, setUsername] = useState(null);
  const [pwd, setPwd] = useState(null);
  const [email, setEmail] = useState(null);

  const submitHandler = (e) => {
    e.preventDefault();
    const formData = {username: username, password: pwd, email: email};
    axios.post(`${import.meta.env.VITE_SERVER_URL}/register`, formData, {
      headers: {'Content-Type': 'application/json'}
    }).then(() => {
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
            className="entry-page__input"
            onChange={(e) => setUsername(e.target.value)}
            required
          />
        </label>
        <label>
          E-mail:
          <input
            type="email"
            name="email"
            className="entry-page__input"
            onChange={(e) => setEmail(e.target.value)}
            required
          />
        </label>
        <label>
          Password:
          <input
            type="password"
            name="password"
            className="entry-page__input"
            onChange={(e) => setPwd(e.target.value)}
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
