import React from 'react'
import ReactDOM from 'react-dom/client'
import {BrowserRouter} from 'react-router-dom'
import {AuthProvider} from "./context/AuthProvider.jsx";
import {GameStateProvider} from "./context/GameStateContext.jsx";
import ErrorBoundary from "./util/components/ErrorBoundary.jsx";
import AppRoutes from "./routes.jsx";
import './index.css'

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <ErrorBoundary>
      <AuthProvider>
        <GameStateProvider>
          <BrowserRouter>
            <AppRoutes />
          </BrowserRouter>
        </GameStateProvider>
      </AuthProvider>
    </ErrorBoundary>
  </React.StrictMode>
)
