import {MainPage} from "../../pages/MainPage/MainPage.jsx";
import EntryPage from "../../pages/EntryPage/EntryPage.jsx";
import {useAuth} from "../../context/AuthProvider.jsx";

export const RequireAuth = () => {
    const auth = useAuth();
    return (
        // <MainPage/>
        auth?.user ? <MainPage/> : <EntryPage/>
    );
}

export default RequireAuth;