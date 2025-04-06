import {createContext, useContext, useState} from "react";
import axios, {HttpStatusCode} from "axios";

const authContext = createContext();
const TOKEN_KEY = "jwt_token";

// Create axios instance with default config
const api = axios.create({
    baseURL: '/',
});

api.interceptors.request.use((config) => {
    const token = localStorage.getItem(TOKEN_KEY);
    if (token) {
        config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
});


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
        setIsLoading(true);
        axios.post(
            "/login",
            {"username": username, "password": password},
            {headers: {'Content-Type': 'application/json', 'Access-Control-Allow-Origin': '*'}}
        ).then(res => {
            if (res.status === HttpStatusCode.Ok) {
                const {access_token, nickname} = res.data;
                localStorage.setItem(TOKEN_KEY, access_token);
                setUser(nickname);
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