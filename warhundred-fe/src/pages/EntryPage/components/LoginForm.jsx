import {useState} from "react";
// import {useNavigate} from "react-router-dom"; // Uncomment when navigation is needed
import {useAuth} from "../../../context/AuthProvider.jsx";
import "../EntryPage.css"

export const LoginForm = () => {
  const [username, setUsername] = useState(null);
  const [pwd, setPwd] = useState(null);

  const auth = useAuth();
  // const nav = useNavigate(); // Uncomment when navigation is needed

  const submitHandler = (e) => {
    e.preventDefault();

    auth.login(username, pwd);
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
          value="Login"
          className="entry-page__submit"
        />
      </form>
    </>
  )
}
