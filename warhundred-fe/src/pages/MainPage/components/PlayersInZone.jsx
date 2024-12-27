import {useState} from "react";

export const PlayersInZone = () => {
  const testState = [{username: "Playa", level: 10}, {username: "Sayaplayer", level: 7}]
  const [players, setPlayers] = useState(testState);

  // TODO: websockets (socket-io) with useEffect() hook.


  return (
    <div className="cnt zone-players-wrapper">
      {players.map((player, idx) => (
        <div className="zone-player-box" key={idx}>
          <div className="zone-player-private">&gt;&gt;&nbsp;</div>
          <div>{player.username} [{player.level}]</div>
          <a href="/" className="zone-player-private">&nbsp;&#9856;</a>
        </div>
      ))}
    </div>
  );
}