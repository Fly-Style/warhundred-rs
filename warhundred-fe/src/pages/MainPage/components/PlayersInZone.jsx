import {useState} from "react";

export const PlayersInZone = () => {
  const testState = ["Playa", "Sayaplayer"]
  const [players, setPlayers] = useState(testState);

  // TODO: websockets (socket-io) with useEffect() hook.

  return (
    <div className="cnt zone-players-wrapper">
      {players.map((player, idx) => (
        <div className="zone-player-box" key={idx}>
          <div>&gt;&gt;&nbsp;</div>
          <div>{player}</div>
          <a href="/">(I)</a>
        </div>
      ))}
    </div>
  );
}