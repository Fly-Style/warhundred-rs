import "./MainPage.css"
import {Chat} from './components/Chat.jsx'
import {PlayersInZone} from "./components/PlayersInZone.jsx";
import {PlayerMiniProfile} from "./components/PlayerMiniProfile.jsx";
import GameWindow from "../GameWindow/GameWindow.jsx";
import {useAuth} from "../../context/AuthProvider.jsx";
import { Button } from "../../components/ui";

export const MainPage = () => {
    const auth = useAuth();

    const logoutHandler = () => {
        auth.logout();
    }

    return (
        <div className="parent">
            <div className="header">Header</div>
            <GameWindow className="cnt game-window-wrap"/>
            <div className="cnt hidden-area-wrap">
                <Button 
                    variant="danger" 
                    buttonProps={{ onClick: logoutHandler }}
                    className="logout-btn"
                >
                    Logout
                </Button>
            </div>
            <div className="locations-list-wrap">
                <div className="locations-header">
                    <h3>LOCATIONS</h3>
                </div>
                <PlayerMiniProfile useTestData={true} />
            </div>
            <Chat useTestData={true} />
            <PlayersInZone useTestData={true} />
        </div>
    )
}
