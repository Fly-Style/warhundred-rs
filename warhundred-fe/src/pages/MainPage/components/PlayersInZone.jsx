import {useState, useEffect} from "react";
import axios from "axios";
import PropTypes from 'prop-types';

// Test data for development and testing
const testPlayers = [
  {nickname: "ArcherQueen", level: 5,},
  {nickname: "DarkWizard", level: 6},
  {nickname: "HealerGirl", level: 3},
  {nickname: "TankMaster", level: 7},
  {nickname: "Warrior123", level: 7}
];

export const PlayersInZone = ({useTestData = false}) => {
  const [players, setPlayers] = useState(useTestData ? testPlayers : []);
  const [error, setError] = useState(null);
  const [isLoading, setIsLoading] = useState(!useTestData);

  useEffect(() => {
    // Skip API calls if using test data
    if (useTestData) {
      setIsLoading(false);
      return;
    }

    // Function to fetch players in the zone
    const fetchPlayersInZone = async () => {
      try {
        setError(null);
        setIsLoading(true);

        // Make API call to get players in the zone
        // TODO: Make it properly for zone, not for all players online.
        const response = await axios.get('/zone/players');

        if (response.status === 200 && Array.isArray(response.data)) {
          setPlayers(response.data);
        } else {
          setPlayers([]);
          setError("Failed to load players data");
        }
      } catch (err) {
        setError("Error connecting to server");
        setPlayers([]);
      } finally {
        setIsLoading(false);
      }
    };

    // Initial fetch when a component mounts
    fetchPlayersInZone();

    // Setup interval to fetch players every ten seconds
    const intervalId = setInterval(fetchPlayersInZone, 10000);

    // Clean up function to the clear interval when the component unmounts
    return () => {
      clearInterval(intervalId);
    };
  }, [useTestData]); // Add useTestData to dependency array

  return (
    <div className="zone-players-wrapper">
      <div className="zone-players-header">
        <h3>Players in Zone</h3>
        <span className="player-count">{players.length} online</span>
      </div>

      {isLoading && <div className="loading-indicator">Loading players...</div>}
      {error && <div className="error-message">{error}</div>}

      <div className="zone-players-list">
        {players.length > 0 ? (
          players.map((player) => (
            <div
              className="zone-player-box"
              key={player.id || player.nickname}
            >
              <div className="player-avatar">
                {player.nickname ? player.nickname.charAt(0).toUpperCase() : '?'}
              </div>
              <div className="player-info">
                <div className="player-name">{player.nickname}[{player.level}]</div>
              </div>
              <div className="player-actions">
                <button className="action-button message-btn" title="Send message">
                  <span role="img" aria-label="Message">✉️</span>
                </button>
                <button className="action-button profile-btn" title="View profile">
                  <span role="img" aria-label="Profile">👤</span>
                </button>
              </div>
            </div>
          ))
        ) : (
          !isLoading && !error && <div className="empty-state">No players in this zone</div>
        )}
      </div>
    </div>
  );
};

PlayersInZone.propTypes = {
  useTestData: PropTypes.bool
};
