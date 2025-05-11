import { describe, it, expect, vi } from 'vitest';
import { handleFormChange } from './utils';

describe('utils', () => {
  describe('handleFormChange', () => {
    it('should update form data with new field value', () => {
      // Arrange
      const event = {
        target: {
          name: 'username',
          value: 'testuser'
        }
      };
      const formDataSetter = vi.fn();
      
      // Act
      handleFormChange(event, formDataSetter);
      
      // Assert
      expect(formDataSetter).toHaveBeenCalledTimes(1);
      
      // Get the callback function passed to formDataSetter
      const callback = formDataSetter.mock.calls[0][0];
      
      // Test that the callback correctly updates the state
      const prevState = { email: 'test@example.com' };
      const newState = callback(prevState);
      
      expect(newState).toEqual({
        email: 'test@example.com',
        username: 'testuser'
      });
    });
  });
});