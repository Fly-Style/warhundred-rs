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
                setError(null); // Reset error state on new fetch attempt

                // If we have a real API endpoint, we would use it here
                // const response = await playerService.getPlayerProfile(auth.user);
                // setPlayerData(response.data);

                // For now, using test data
                timeoutId = setTimeout(() => {
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
                        intelligence: 7,
                        wisdom: 6,
                        constitution: 9,
                        charisma: 5,
                        inventory: [
                            {id: 1, name: "Wooden Bow", type: "Weapon", quality: "Common"},
                            {id: 2, name: "Leather Armor", type: "Armor", quality: "Common"},
                            {id: 3, name: "Health Potion", type: "Consumable", quality: "Common", quantity: 3}
                        ],
                        skills: [
                            {id: 1, name: "Archery", level: 3, progress: 65, maxProgress: 100},
                            {id: 2, name: "Stealth", level: 2, progress: 40, maxProgress: 100},
                            {id: 3, name: "Survival", level: 1, progress: 25, maxProgress: 100}
                        ],
                        achievements: [
                            {
                                id: 1,
                                name: "First Blood",
                                description: "Defeat your first enemy",
                                completed: true,
                                progress: 1,
                                maxProgress: 1
                            },
                            {
                                id: 2,
                                name: "Collector",
                                description: "Collect 50 items",
                                completed: false,
                                progress: 24,
                                maxProgress: 50
                            },
                            {
                                id: 3,
                                name: "Explorer",
                                description: "Travel 5000 meters",
                                completed: false,
                                progress: 1500,
                                maxProgress: 5000
                            }
                        ],
                        statistics: {
                            enemiesDefeated: 12,
                            questsCompleted: 3,
                            itemsCollected: 24,
                            distanceTraveled: 1500,
                            timePlayedMinutes: 120
                        }
                    });
                    setLoading(false);
                }, 1000);
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
        nickname, level, rank, spec, health, maxHealth, stamina, maxStamina, experience,
        nextLevelExperience, strength, dexterity, intelligence, inventory, statistics
    } = playerData;

    return (
        <div className="player-profile-page">
            <div className="player-profile-header">
                <h1>{nickname}</h1>
                <div className="player-profile-basic-info">
                    <span className="player-level">Level: {level}</span>
                    <span className="player-rank">Rank: {rank}</span>
                    <span className="player-class">Class: {spec}</span>
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
                        <div className="player-stat">
                            <div className="stat-label">Experience</div>
                            <div className="stat-bar-container">
                                <div
                                    className="stat-bar experience-bar"
                                    style={{width: `${(experience / nextLevelExperience) * 100}%`}}
                                ></div>
                                <div className="stat-value">{experience}/{nextLevelExperience}</div>
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
                            <span className="attribute-label">INT</span>
                            <span className="attribute-value">{intelligence}</span>
                        </div>
                    </div>
                </div>

                {/* Statistics - span 1 column */}
                <div className="player-profile-section">
                    <h2>Statistics</h2>
                    <div className="player-statistics-grid">
                        <div className="player-statistic">
                            <span className="statistic-label">Enemies Defeated</span>
                            <span className="statistic-value">{statistics.enemiesDefeated}</span>
                        </div>
                        <div className="player-statistic">
                            <span className="statistic-label">Quests Completed</span>
                            <span className="statistic-value">{statistics.questsCompleted}</span>
                        </div>
                        <div className="player-statistic">
                            <span className="statistic-label">Items Collected</span>
                            <span className="statistic-value">{statistics.itemsCollected}</span>
                        </div>
                        <div className="player-statistic">
                            <span className="statistic-label">Distance</span>
                            <span className="statistic-value">{statistics.distanceTraveled}m</span>
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
