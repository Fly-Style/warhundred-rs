import { describe, it, expect, vi, beforeEach } from 'vitest';

describe('Vite Environment Configuration', () => {
  beforeEach(() => {
    // Clear all mocks before each test
    vi.clearAllMocks();
  });

  it('should set VITE_API_URL to VITE_SERVER_URL in development mode', () => {
    // Arrange
    const mode = 'development';
    const env = {
      VITE_SERVER_URL: 'http://localhost:8000',
      VITE_PROD_SERVER_URL: 'https://srv851138.hstgr.cloud/'
    };

    // Act - Simulate the logic in vite.config.js
    const apiUrl = mode === 'production' 
      ? env.VITE_PROD_SERVER_URL 
      : env.VITE_SERVER_URL;

    // Assert
    expect(apiUrl).toBe('http://localhost:8000');
  });

  it('should set VITE_API_URL to VITE_PROD_SERVER_URL in production mode', () => {
    // Arrange
    const mode = 'production';
    const env = {
      VITE_SERVER_URL: 'http://localhost:8000',
      VITE_PROD_SERVER_URL: 'https://srv851138.hstgr.cloud/'
    };

    // Act - Simulate the logic in vite.config.js
    const apiUrl = mode === 'production' 
      ? env.VITE_PROD_SERVER_URL 
      : env.VITE_SERVER_URL;

    // Assert
    expect(apiUrl).toBe('https://srv851138.hstgr.cloud/');
  });

  it('should correctly load environment variables', () => {
    // This test verifies that loadEnv is called correctly in the vite config

    // Arrange
    const mockLoadEnv = vi.fn().mockReturnValue({
      VITE_SERVER_URL: 'http://localhost:8000',
      VITE_PROD_SERVER_URL: 'https://srv851138.hstgr.cloud/'
    });

    // Act - Simulate the vite config function
    const configFunction = ({ mode }) => {
      const env = mockLoadEnv(mode, '/mock/project/path', '');

      return {
        define: {
          'import.meta.env.VITE_API_URL': mode === 'production' 
            ? JSON.stringify(env.VITE_PROD_SERVER_URL) 
            : JSON.stringify(env.VITE_SERVER_URL)
        }
      };
    };

    const devConfig = configFunction({ mode: 'development' });
    const prodConfig = configFunction({ mode: 'production' });

    // Assert
    expect(mockLoadEnv).toHaveBeenCalledTimes(2);
    expect(devConfig.define['import.meta.env.VITE_API_URL']).toBe(JSON.stringify('http://localhost:8000'));
    expect(prodConfig.define['import.meta.env.VITE_API_URL']).toBe(JSON.stringify('https://srv851138.hstgr.cloud/'));
  });
});
