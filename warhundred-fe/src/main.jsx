import React from 'react'
import ReactDOM from 'react-dom/client'
import {BrowserRouter, Routes, Route} from 'react-router-dom'
import './index.css'
import "./util/components/RequireAuth.jsx";
import {AuthProvider} from "./context/AuthProvider.jsx";
import {EntryPage} from "./pages/EntryPage/EntryPage.jsx";
import MainPage from "./pages/MainPage/MainPage.jsx";
import Missing from "./util/components/Missing.jsx";
import RequireAuth from "./util/components/RequireAuth.jsx";

ReactDOM.createRoot(document.getElementById('root')).render(
    <React.StrictMode>
      <BrowserRouter>
        {/*<AuthProvider>*/}
          <Routes>
            {/*TODO: do auth with JWT and put it into the storage*/}
            {/*<Route path="/" element={<EntryPage/>}/>*/}
            {/*<Route element={<RequireAuth/>}>*/}
            <Route path="/" element={<MainPage/>}/>
            {/*</Route>*/}
            <Route path="*" element={<Missing/>}/>
          </Routes>
        {/*</AuthProvider>*/}
      </BrowserRouter>
    </React.StrictMode>
)
