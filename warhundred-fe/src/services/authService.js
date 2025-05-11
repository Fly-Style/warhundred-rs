import axios, { HttpStatusCode } from "axios";

const TOKEN_KEY = "jwt_token";

// Create axios instance with default config
const api = axios.create({
    baseURL: '/',
});

// Add authorization token to requests
api.interceptors.request.use((config) => {
    const token = localStorage.getItem(TOKEN_KEY);
    if (token) {
        config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
});

/**
 * Login a user with username and password
 * @param {string} username - The username
 * @param {string} password - The password
 * @returns {Promise} - Promise that resolves with the response data
 */
export const login = async (username, password) => {
    const response = await axios.post(
        "/login",
        { "username": username, "password": password },
        { headers: { 'Content-Type': 'application/json', 'Access-Control-Allow-Origin': '*' } }
    );
    
    if (response.status === HttpStatusCode.Ok) {
        const { access_token } = response.data;
        localStorage.setItem(TOKEN_KEY, access_token);
    }
    
    return response;
};

/**
 * Logout the current user
 * @param {string} nickname - The user's nickname
 * @returns {Promise} - Promise that resolves when logout is complete
 */
export const logout = async (nickname) => {
    const accessToken = localStorage.getItem(TOKEN_KEY);
    
    // Remove the token from localStorage
    if (accessToken !== undefined && accessToken !== null) {
        localStorage.removeItem(TOKEN_KEY);
    }
    
    // If there's a user, make the logout request
    if (nickname) {
        return axios.post(
            "/logout",
            { "nickname": nickname, "access_token": accessToken },
            { headers: { 'Content-Type': 'application/json' } }
        );
    }
    
    return Promise.resolve();
};

/**
 * Get the current authentication token
 * @returns {string|null} - The current token or null if not authenticated
 */
export const getToken = () => {
    return localStorage.getItem(TOKEN_KEY);
};

/**
 * Check if the user is authenticated
 * @returns {boolean} - True if authenticated, false otherwise
 */
export const isAuthenticated = () => {
    return !!getToken();
};

export default {
    login,
    logout,
    getToken,
    isAuthenticated,
    api
};