import authService from './authService';

/**
 * Get player profile data
 * @param {string} nickname - The player's nickname
 * @returns {Promise} - Promise that resolves with the player profile data
 */
export const getPlayerProfile = async (nickname) => {
  try {
    // Use the configured axios instance from authService to ensure auth headers are included
    const response = await authService.api.get(`/profile/${nickname}`);
    return response;
  } catch (error) {
    console.error('Error fetching player profile:', error);
    throw error;
  }
};

/**
 * Update player profile data
 * @param {string} nickname - The player's nickname
 * @param {Object} profileData - The updated profile data
 * @returns {Promise} - Promise that resolves with the updated player profile data
 */
export const updatePlayerProfile = async (nickname, profileData) => {
  try {
    const response = await authService.api.put(`/profile/${nickname}`, profileData);
    return response;
  } catch (error) {
    console.error('Error updating player profile:', error);
    throw error;
  }
};

/**
 * Get player inventory
 * @param {string} nickname - The player's nickname
 * @returns {Promise} - Promise that resolves with the player's inventory
 */
export const getPlayerInventory = async (nickname) => {
  try {
    const response = await authService.api.get(`/player/inventory/${nickname}`);
    return response;
  } catch (error) {
    console.error('Error fetching player inventory:', error);
    throw error;
  }
};

/**
 * Get player skills
 * @param {string} nickname - The player's nickname
 * @returns {Promise} - Promise that resolves with the player's skills
 */
export const getPlayerSkills = async (nickname) => {
  try {
    const response = await authService.api.get(`/player/skills/${nickname}`);
    return response;
  } catch (error) {
    console.error('Error fetching player skills:', error);
    throw error;
  }
};

/**
 * Get player achievements
 * @param {string} nickname - The player's nickname
 * @returns {Promise} - Promise that resolves with the player's achievements
 */
export const getPlayerAchievements = async (nickname) => {
  try {
    const response = await authService.api.get(`/player/achievements/${nickname}`);
    return response;
  } catch (error) {
    console.error('Error fetching player achievements:', error);
    throw error;
  }
};

/**
 * Get player statistics
 * @param {string} nickname - The player's nickname
 * @returns {Promise} - Promise that resolves with the player's statistics
 */
export const getPlayerStatistics = async (nickname) => {
  try {
    const response = await authService.api.get(`/player/statistics/${nickname}`);
    return response;
  } catch (error) {
    console.error('Error fetching player statistics:', error);
    throw error;
  }
};

export default {
  getPlayerProfile,
  updatePlayerProfile,
  getPlayerInventory,
  getPlayerSkills,
  getPlayerAchievements,
  getPlayerStatistics
};
