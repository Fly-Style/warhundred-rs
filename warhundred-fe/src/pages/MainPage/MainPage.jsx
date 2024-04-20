import "./MainPage.css"
import {Chat} from './components/Chat.jsx'
import {PlayersInZone} from "./components/PlayersInZone.jsx";

export const MainPage = () => {
  return (
      <div className="parent">
          <div className="header">Header</div>
          <div className="cnt game-window-wrap">GAME WINDOW</div>
          <div className="cnt hidden-area-wrap">HIDDEN</div>
          <div className="locations-list-wrap">LOCATIONS</div>
          <Chat className="cnt chat-parent">CHAT</Chat>
          <PlayersInZone>ZONE PLAYERS</PlayersInZone>
      </div>
  )
}
export default MainPage;
