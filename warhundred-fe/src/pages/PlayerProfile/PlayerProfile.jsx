import {useState, useEffect} from 'react';
import {useAuth} from '../../context/AuthProvider.jsx';
// import { playerService } from '../../services/playerService';
import './PlayerProfile.css';

/**
 * Player Profile Page Component
 * Displays detailed information about the player
 * @returns {JSX.Element} - Player Profile page component
 */
export const PlayerProfile = () => {
    const auth = useAuth();
    const [playerData, setPlayerData] = useState(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);

    useEffect(() => {
        let timeoutId;

        const fetchPlayerData = async () => {
            try {
                setLoading(true);
                setError(null); // Reset error state on a new fetch attempt

                // If we have a real API endpoint, we would use it here
                // const response = await playerService.getPlayerProfile(auth.user);
                // setPlayerData(response.data);

                setPlayerData({
                    nickname: auth.user || "YourCharacter",
                    level: 5,
                    rank: "Novice",
                    spec: "Archer",
                    health: 75,
                    maxHealth: 100,
                    stamina: 60,
                    maxStamina: 100,
                    experience: 65,
                    nextLevelExperience: 100,
                    strength: 8,
                    dexterity: 12,
                    physique: 7,
                    luck: 7,
                    intellect: 7,
                    inventory: [],
                    statistics: {
                        battlesWon: 12,
                        battlesLost: 3,
                        timePlayedMinutes: 120
                    }
                });
                setLoading(false);
            } catch (err) {
                setError("Failed to load player data. Please try again later.");
                setLoading(false);
                console.error("Error fetching player data:", err);
            }
        };

        fetchPlayerData();

        // Cleanup function to clear the timeout if the component unmounts
        return () => {
            if (timeoutId) {
                clearTimeout(timeoutId);
            }
        };
    }, [auth.user]);

    if (loading) {
        return (
            <div className="player-profile-loading" role="status" aria-live="polite">
                <span>Loading player data...</span>
            </div>
        );
    }

    if (error) {
        return (
            <div className="player-profile-error" role="alert" aria-live="assertive">
                {error}
            </div>
        );
    }

    if (!playerData) {
        return (
            <div className="player-profile-error" role="alert" aria-live="assertive">
                No player data available
            </div>
        );
    }

    const {
        nickname, level, rank, spec, health, maxHealth, stamina, maxStamina,
        strength, dexterity, physique, luck, intellect, inventory, statistics
    } = playerData;

    return (
        <div className="player-profile-page">
            <div className="player-profile-header">
                <h1>{nickname}</h1>
                <div className="player-profile-basic-info">
                    <span className="player-rank">
                        <div className="rank-image-placeholder"></div>
                        Rank: {rank}
                    </span>
                    <span className="player-level">Level: {level}</span>
                    <span className="player-class">
                        <div className="class-image-placeholder"></div>
                        Class: {spec}
                    </span>
                </div>
            </div>

            <div className="player-profile-content" role="region" aria-label="Player profile details">
                {/* Character Stats - span 1 column */}
                <div className="player-profile-section">
                    <h2>Character Stats</h2>
                    <div className="player-stats-grid">
                        <div className="player-stat">
                            <div className="stat-label">Health</div>
                            <div className="stat-bar-container">
                                <div
                                    className="stat-bar health-bar"
                                    style={{width: `${(health / maxHealth) * 100}%`}}
                                    role="progressbar"
                                    aria-valuenow={health}
                                    aria-valuemin="0"
                                    aria-valuemax={maxHealth}
                                    aria-label="Health progress"
                                ></div>
                                <div className="stat-value">{health}/{maxHealth}</div>
                            </div>
                        </div>
                        <div className="player-stat">
                            <div className="stat-label">Stamina</div>
                            <div className="stat-bar-container">
                                <div
                                    className="stat-bar stamina-bar"
                                    style={{width: `${(stamina / maxStamina) * 100}%`}}
                                    role="progressbar"
                                    aria-valuenow={stamina}
                                    aria-valuemin="0"
                                    aria-valuemax={maxStamina}
                                    aria-label="Stamina progress"
                                ></div>
                                <div className="stat-value">{stamina}/{maxStamina}</div>
                            </div>
                        </div>
                    </div>
                </div>

                {/* Attributes - span 1 column */}
                <div className="player-profile-section">
                    <h2>Attributes</h2>
                    <div className="player-attributes-grid">
                        <div className="player-attribute">
                            <span className="attribute-label">STR</span>
                            <span className="attribute-value">{strength}</span>
                        </div>
                        <div className="player-attribute">
                            <span className="attribute-label">DEX</span>
                            <span className="attribute-value">{dexterity}</span>
                        </div>
                        <div className="player-attribute">
                            <span className="attribute-label">PHYS</span>
                            <span className="attribute-value">{physique}</span>
                        </div>
                        <div className="player-attribute">
                            <span className="attribute-label">LUCK</span>
                            <span className="attribute-value">{luck}</span>
                        </div>
                        <div className="player-attribute">
                            <span className="attribute-label">INT</span>
                            <span className="attribute-value">{intellect}</span>
                        </div>
                    </div>
                </div>

                {/* Statistics - span 1 column */}
                <div className="player-profile-section">
                    <h2>Statistics</h2>
                    <div className="player-statistics-grid">
                        <div className="player-statistic">
                            <span className="statistic-label">Battles won</span>
                            <span className="statistic-value">{statistics.battlesWon}</span>
                        </div>
                        <div className="player-statistic">
                            <span className="statistic-label">Battles lost</span>
                            <span className="statistic-value">{statistics.battlesLost}</span>
                        </div>
                        <div className="player-statistic">
                            <span className="statistic-label">Time Played</span>
                            <span
                                className="statistic-value">{Math.floor(statistics.timePlayedMinutes / 60)}h {statistics.timePlayedMinutes % 60}m</span>
                        </div>
                    </div>
                </div>

                {/* Inventory - span 1 column */}
                <div className="player-profile-section">
                    <h2>Inventory</h2>
                    {inventory.length > 0 ? (
                        <div className="player-inventory-list">
                            {inventory.map(item => (
                                <div key={item.id} className="inventory-item">
                                    <span className="item-name">{item.name}</span>
                                    <span className="item-type">{item.type}</span>
                                    <span className="item-quality">{item.quality}</span>
                                    {item.quantity && <span className="item-quantity">x{item.quantity}</span>}
                                </div>
                            ))}
                        </div>
                    ) : (
                        <div className="empty-state">No items in inventory</div>
                    )}
                </div>
            </div>
        </div>
    );
};

PlayerProfile.propTypes = {
    // If the component receives props in the future, define them here
};

export default PlayerProfile;
