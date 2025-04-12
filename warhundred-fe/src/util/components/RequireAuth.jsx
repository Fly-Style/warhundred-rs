import {MainPage} from "../../pages/MainPage/MainPage.jsx";
import EntryPage from "../../pages/EntryPage/EntryPage.jsx";
import {useAuth} from "../../context/AuthProvider.jsx";

export const RequireAuth = () => {
    const auth = useAuth();
    console.log(auth);
    return (
        auth?.user ? <MainPage/> : <EntryPage/>
    );
}

export default RequireAuth;