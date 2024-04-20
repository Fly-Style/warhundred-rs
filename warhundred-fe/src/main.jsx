import React from 'react'
import ReactDOM from 'react-dom/client'
import {BrowserRouter, Routes, Route} from 'react-router-dom'
import {AuthProvider} from "./context/AuthProvider.jsx";
import RequireAuth from "./util/components/RequireAuth.jsx";
import Missing from "./util/components/Missing.jsx";
import './index.css'

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <AuthProvider>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<RequireAuth/>}/>
          <Route path="*" element={<Missing/>}/>
        </Routes>
      </BrowserRouter>
    </AuthProvider>
  </React.StrictMode>
)
