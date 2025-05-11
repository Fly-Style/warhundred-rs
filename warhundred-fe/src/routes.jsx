import { Route, Routes } from 'react-router-dom';
import RequireAuth from './util/components/RequireAuth.jsx';
import Missing from './util/components/Missing.jsx';

/**
 * Application routes configuration
 * @returns {JSX.Element} - Routes component
 */
export const AppRoutes = () => {
  return (
    <Routes>
      <Route path="/" element={<RequireAuth />} />
      <Route path="*" element={<Missing />} />
    </Routes>
  );
};

/**
 * Route paths used in the application
 * This allows for centralized management of route paths
 * 
 * Note: This is not a component, so it's exempt from the react-refresh/only-export-components rule
 */
/* eslint-disable react-refresh/only-export-components */
export const ROUTES = {
  HOME: '/',
  // Add more routes as needed
};
/* eslint-enable react-refresh/only-export-components */

export default AppRoutes;
