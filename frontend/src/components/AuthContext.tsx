import React, { createContext, useContext, useState, useEffect } from 'react';
import type { ReactNode } from 'react';
import { OpenAPI } from '../api/generated';

interface AuthContextType {
    token: string | null;
    login: (token: string) => void;
    logout: () => void;
    isAuthenticated: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
    const [token, setToken] = useState<string | null>(() => {
        return localStorage.getItem('token');
    });

    useEffect(() => {
        // Just keeping this for cross-tab sync if needed, though mostly handled synchronously now
        if (token) {
            localStorage.setItem('token', token);
            OpenAPI.TOKEN = token;
        } else {
            localStorage.removeItem('token');
            OpenAPI.TOKEN = undefined;
        }
    }, [token]);

    const login = (newToken: string) => {
        localStorage.setItem('token', newToken);
        OpenAPI.TOKEN = newToken;
        setToken(newToken);
    };

    const logout = () => {
        localStorage.removeItem('token');
        OpenAPI.TOKEN = undefined;
        setToken(null);
    };

    return (
        <AuthContext.Provider value={{ token, login, logout, isAuthenticated: !!token }}>
            {children}
        </AuthContext.Provider>
    );
};

export const useAuth = () => {
    const context = useContext(AuthContext);
    if (context === undefined) {
        throw new Error('useAuth must be used within an AuthProvider');
    }
    return context;
};
