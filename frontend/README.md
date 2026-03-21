<<<<<<< Current (Your changes)
=======
# Frontend (Daily Tracker)

React + TypeScript + Vite frontend for the Daily Tracker backend.

## Prerequisites

- Node.js 20+
- Backend running at `http://localhost:8080` (or set custom API URL)

## Setup

```bash
npm install
cp .env.example .env.local
```

## Environment

- `VITE_API_BASE_URL` (default in `.env.example`):
  - `http://localhost:8080/api/v1`

## Development

```bash
npm run dev
```

## Quality + Build

```bash
npm run lint
npm run typecheck
npm run build
```

## OpenAPI Client Generation

The frontend uses generated TypeScript client code from the backend OpenAPI spec:

- input: `../backend/api/openapi.yaml`
- output: `src/api/generated`

Regenerate:

```bash
npm run generate:api
```

## Implemented Screens

- `Topics` page
  - list topics
  - create topic
- `Daily Tracks` page
  - list daily tracks
  - filter by `start_date`, `end_date`, `topic_id`
  - create daily track with client-side `start_time` validation (`:00` or `:30`)

# React + TypeScript + Vite

This template provides a minimal setup to get React working in Vite with HMR and some ESLint rules.

Currently, two official plugins are available:

- [@vitejs/plugin-react](https://github.com/vitejs/vite-plugin-react/blob/main/packages/plugin-react) uses [Oxc](https://oxc.rs)
- [@vitejs/plugin-react-swc](https://github.com/vitejs/vite-plugin-react/blob/main/packages/plugin-react-swc) uses [SWC](https://swc.rs/)

## React Compiler

The React Compiler is not enabled on this template because of its impact on dev & build performances. To add it, see [this documentation](https://react.dev/learn/react-compiler/installation).

## Expanding the ESLint configuration

If you are developing a production application, we recommend updating the configuration to enable type-aware lint rules:

```js
export default defineConfig([
  globalIgnores(['dist']),
  {
    files: ['**/*.{ts,tsx}'],
    extends: [
      // Other configs...

      // Remove tseslint.configs.recommended and replace with this
      tseslint.configs.recommendedTypeChecked,
      // Alternatively, use this for stricter rules
      tseslint.configs.strictTypeChecked,
      // Optionally, add this for stylistic rules
      tseslint.configs.stylisticTypeChecked,

      // Other configs...
    ],
    languageOptions: {
      parserOptions: {
        project: ['./tsconfig.node.json', './tsconfig.app.json'],
        tsconfigRootDir: import.meta.dirname,
      },
      // other options...
    },
  },
])
```

You can also install [eslint-plugin-react-x](https://github.com/Rel1cx/eslint-react/tree/main/packages/plugins/eslint-plugin-react-x) and [eslint-plugin-react-dom](https://github.com/Rel1cx/eslint-react/tree/main/packages/plugins/eslint-plugin-react-dom) for React-specific lint rules:

```js
// eslint.config.js
import reactX from 'eslint-plugin-react-x'
import reactDom from 'eslint-plugin-react-dom'

export default defineConfig([
  globalIgnores(['dist']),
  {
    files: ['**/*.{ts,tsx}'],
    extends: [
      // Other configs...
      // Enable lint rules for React
      reactX.configs['recommended-typescript'],
      // Enable lint rules for React DOM
      reactDom.configs.recommended,
    ],
    languageOptions: {
      parserOptions: {
        project: ['./tsconfig.node.json', './tsconfig.app.json'],
        tsconfigRootDir: import.meta.dirname,
      },
      // other options...
    },
  },
])
```
>>>>>>> Incoming (Background Agent changes)
