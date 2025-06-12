import {MainPage} from "../../pages/MainPage/MainPage.jsx";
import EntryPage from "../../pages/EntryPage/EntryPage.jsx"; // Uncomment when implementing conditional rendering
import {useAuth} from "../../context/AuthProvider.jsx"; // Uncomment when implementing conditional rendering

export const RequireAuth = () => {
    const auth = useAuth(); // Uncomment when implementing conditional rendering
    return (
        // <MainPage/>
        auth?.user ? <MainPage/> : <EntryPage/>
    );
}

export default RequireAuth;
