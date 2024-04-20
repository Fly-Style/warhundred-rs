import {createContext, useContext, useState} from "react";
import axios, {HttpStatusCode} from "axios";

const authContext = createContext();

export const AuthProvider = ({children}) => {
  const auth = useProvideAuth()
  return <authContext.Provider value={auth}>
    {children}
  </authContext.Provider>
}

export const useAuth = () => {
  return useContext(authContext);
}

function useProvideAuth() {
  const [user, setUser] = useState(null);
  const [errors, setErrors] = useState([]);
  const [isLoading, setIsLoading] = useState(false);

  function login(username, password) {
    // TODO: extract JWT token and use it for session.
    setIsLoading(true);
    axios.post(
      "/login",
      {"username": username, "password": password},
      {headers: {'Content-Type': 'application/json'}}
    ).then(res => {
      if (res.status === HttpStatusCode.Ok) {
        setUser(username);
      } else {
        setErrors(JSON.parse(res.data));
      }
    }).catch(err => setErrors(err));
  }

  function logout(username) {
    setErrors([]);
    axios.post("/logout", {"username": username}, {headers: {'Content-Type': 'application/json'}})
      .then(_ => setUser(null))
      .catch(err => setErrors(err))
  }

  return {
    user,
    login,
    logout,
    isLoading,
    errors
  }
}