import {createContext, useContext, useState} from "react";
import PropTypes from 'prop-types';
import authService from "../services/authService";

const authContext = createContext();

export const AuthProvider = ({children}) => {
    const auth = useProvideAuth()
    return <authContext.Provider value={auth}>
        {children}
    </authContext.Provider>
}

AuthProvider.propTypes = {
    children: PropTypes.node.isRequired
};

// Moved to a separate named export to avoid react-refresh warning
// This is a hook, not a component, so it's exempt from the react-refresh/only-export-components rule
/* eslint-disable react-refresh/only-export-components */
export const useAuth = () => {
    return useContext(authContext);
}
/* eslint-enable react-refresh/only-export-components */

function useProvideAuth() {
    const [user, setUser] = useState(null);
    const [errors, setErrors] = useState([]);
    const [isLoading, setIsLoading] = useState(false);

    function login(username, password) {
        setIsLoading(true);
        authService.login(username, password)
            .then(res => {
                const {nickname} = res.data;
                setUser(nickname);
                setIsLoading(false);
            })
            .catch(err => {
                setErrors(err);
                setIsLoading(false);
            });
    }

    function logout() {
        setIsLoading(true);
        setErrors([]);

        // Get the current user's information if needed for the API call
        const currentUser = user;

        authService.logout(currentUser)
            .then(() => {
                setUser(null);
            })
            .catch(err => {
                setErrors(err);
            })
            .finally(() => {
                setIsLoading(false);
            });
    }

    return {
        user,
        login,
        logout,
        isLoading,
        errors
    }
}
