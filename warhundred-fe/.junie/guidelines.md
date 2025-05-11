# Warhundred Frontend Development Guidelines

This document provides essential information for developers working on the Warhundred frontend project.

## Build/Configuration Instructions

### Prerequisites
- Node.js (version 18 or higher recommended)
- npm (comes with Node.js)

### Setup
1. Clone the repository
2. Install dependencies:
   ```bash
   npm install
   ```

### Development
To start the development server with hot module replacement (HMR):
```bash
npm run dev
```

### Building for Production
To build the application for production:
```bash
npm run build
```

The build output will be placed in the `../public` directory (relative to the frontend project root), as configured in `vite.config.js`.

### Preview Production Build
To preview the production build locally:
```bash
npm run preview
```

## Testing Information

### Testing Framework
The project uses Vitest as the testing framework, along with React Testing Library for component testing.

### Running Tests
To run all tests once:
```bash
npm test
```

To run tests in watch mode during development:
```bash
npm run test:watch
```

### Test Structure
- Test files should be placed next to the files they test with a `.test.js` or `.test.jsx` extension
- The `src/test/setup.js` file configures the testing environment

### Writing Tests
Tests follow the Arrange-Act-Assert pattern:

```javascript
import { describe, it, expect, vi } from 'vitest';
import { YourComponent } from './YourComponent';
import { render, screen } from '@testing-library/react';

describe('YourComponent', () => {
  it('should render correctly', () => {
    // Arrange
    render(<YourComponent />);
    
    // Act
    const element = screen.getByText('Expected Text');
    
    // Assert
    expect(element).toBeInTheDocument();
  });
});
```

### Mocking
Use Vitest's mocking capabilities for mocking dependencies:

```javascript
import { vi } from 'vitest';

// Mock a function
const mockFn = vi.fn();

// Mock a module
vi.mock('./path/to/module', () => ({
  someFunction: vi.fn(),
  someValue: 'mocked value'
}));
```

## Additional Development Information

### Tests and linting
Always run tests and linting before committing code.

### Project Structure
- `src/pages/` - Contains page components
- `src/context/` - React context providers
- `src/util/` - Utility functions
- `src/test/` - Test configuration

### Graphics Rendering
The project uses PixiJS (via @pixi/react) for rendering game graphics. Canvas components can be found in:
- `src/pages/GameWindow/components/TownCanvas.jsx`
- `src/pages/GameWindow/components/BattleCanvas.jsx`

### Code Style
- The project uses ESLint for code linting with React-specific rules
- 2-space indentation (configured in .editorconfig)
- React functional components with hooks are preferred over class components

### Environment Variables
Environment variables are stored in the `.env` file at the project root. In development, you can create a `.env.local` file (which should not be committed to version control) to override values.

### Build System
- Vite is used as the build tool
- The project is configured to output a single file bundle using the `vite-plugin-singlefile` plugin
- SWC is used for fast transpilation via `@vitejs/plugin-react-swc`

### Documentation
- JSDoc is available for documenting code
- Use descriptive component and function names
- Add comments for complex logic