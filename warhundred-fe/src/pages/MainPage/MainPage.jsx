import "./MainPage.css"
import {Chat} from './components/Chat.jsx'
import {PlayersInZone} from "./components/PlayersInZone.jsx";
import GameWindow from "../GameWindow/GameWindow.jsx";

export const MainPage = () => {
  return (
      <div className="parent">
          <div className="header">Header</div>
          <GameWindow className="cnt game-window-wrap"/>
          <div className="cnt hidden-area-wrap">HIDDEN</div>
          <div className="locations-list-wrap">LOCATIONS</div>
          <Chat className="cnt chat-parent">CHAT</Chat>
          <PlayersInZone>ZONE PLAYERS</PlayersInZone>
      </div>
  )
}
