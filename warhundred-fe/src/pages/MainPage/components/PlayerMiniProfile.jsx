import { useState } from 'react';
import PropTypes from 'prop-types';

// Test data for development and testing
const testPlayerData = {
  nickname: "YourCharacter",
  level: 5,
  rank: "Novice",
  spec: "Archer",
  health: 75, // out of 100
  stamina: 60, // out of 100
  experience: 65, // percentage to the next level
};

export const PlayerMiniProfile = ({ useTestData = true }) => {
  // noinspection JSUnusedLocalSymbols
  const [playerData, setPlayerData] = useState(useTestData ? testPlayerData : null);

  // In a real application, you would fetch the player data from an API or context
  // For now, we're using test data

  if (!playerData) {
    return <div className="player-profile-loading">Loading player data...</div>;
  }

  const { nickname, level, rank, spec, health, stamina, experience } = playerData;

  return (
    <div className="player-mini-profile">
      <div className="player-mini-header">
        <h3>Your Character</h3>
      </div>

      <div className="player-mini-content">
        <div className="player-mini-info">
          <div className="player-mini-name">{nickname}</div>
          <div className="player-mini-level">
            <span className="player-level">Rank: {rank}</span>
            <span className="player-level">Level: {level}</span>
            <span className="player-level">Class: {spec}</span>
          </div>

          <div className="player-mini-stats">
            <div className="player-mini-stat">
              <div className="stat-label">
                <span>Health</span>
                <span>{health}/100</span>
              </div>
              <div className="stat-bar-container">
                <div 
                  className="stat-bar health-bar" 
                  style={{ width: `${health}%` }}
                ></div>
              </div>
            </div>

            <div className="player-mini-stat">
              <div className="stat-label">
                <span>Stamina</span>
                <span>{stamina}/100</span>
              </div>
              <div className="stat-bar-container">
                <div 
                  className="stat-bar stamina-bar" 
                  style={{ width: `${stamina}%` }}
                ></div>
              </div>
            </div>

            <div className="player-mini-stat">
              <div className="stat-label">
                <span>Experience</span>
                <span>{experience}%</span>
              </div>
              <div className="stat-bar-container">
                <div 
                  className="stat-bar experience-bar" 
                  style={{ width: `${experience}%` }}
                ></div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

PlayerMiniProfile.propTypes = {
  useTestData: PropTypes.bool
};
