import useAuth from "../../hooks/useAuth.js";
import {MainPage} from "../../pages/MainPage/MainPage.jsx";
import EntryPage from "../../pages/EntryPage/EntryPage.jsx";

export const RequireAuth = () => {
  const {auth} = useAuth();
  return (
      auth?.user
          ? <MainPage/>
          : <EntryPage/>
  );
}

export default RequireAuth;