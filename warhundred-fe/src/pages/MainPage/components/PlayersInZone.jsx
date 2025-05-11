import {useState, useEffect} from "react";
import axios from "axios";

export const PlayersInZone = () => {
    // Left for testing purposes
    const testState = [{username: "Playa", level: 10}, {username: "Sayaplayer", level: 7}]
    const [players, setPlayers] = useState([]);
    const [error, setError] = useState(null);

    useEffect(() => {
        // Function to fetch players in the zone
        const fetchPlayersInZone = async () => {
            try {
                setError(null);

                // Make API call to get players in the zone
                // TODO: Make it properly for zone, not for all players online.
                const res = (await axios.get('/zone/players'))

                if (response.status === 200) {
                    setPlayers(res.data);
                } else {
                setPlayers([...testState]);
                }
            } catch (err) {
                console.error('Error fetching players in zone:', err);
                setError('Failed to fetch players in zone');
            }
        };

        // Initial fetch when component mounts
        fetchPlayersInZone();

        // Setup interval to fetch players every ten seconds
        const intervalId = setInterval(fetchPlayersInZone, 10000);

        // Clean up function to the clear interval when component unmounts
        return () => {
            clearInterval(intervalId);
        };
    }, []); // Empty dependency array means this effect runs once on mount and cleans up on unmount

    return (
        <div className="cnt zone-players-wrapper">
            {error && <div className="error-message">{error}</div>}
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