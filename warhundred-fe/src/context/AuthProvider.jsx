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

    function logout() {
        setIsLoading(true);
        setErrors([]);

        // Get the current user's information if needed for the API call
        const currentUser = user;

        // Remove the token from localStorage immediately
        localStorage.removeItem(TOKEN_KEY);

        // If your backend requires a logout request, you can still make it
        if (currentUser) {
            axios.post(
                "/logout",
                {"username": currentUser},
                {headers: {'Content-Type': 'application/json'}}
            )
                .then(() => {
                    setUser(null);
                })
                .catch(err => {
                    setErrors(err);
                })
                .finally(() => {
                    setIsLoading(false);
                });
        } else {
            // If no user is set, just complete the logout process
            setUser(null);
            setIsLoading(false);
        }
    }

    return {
        user,
        login,
        logout,
        isLoading,
        errors
    }
}