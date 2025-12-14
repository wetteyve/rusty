# @wetteyve/rusty

> A Rust-powered utility package built with **napi-rs**, focused on experimentation, learning, and cross-platform native & WASM development.

[napi-rs](https://napi.rs/) • [nodeJS](https://nodejs.org/en) • [Rust](https://rust-lang.org/) • [WASM](https://webassembly.org/)

## 📚 Docs

- [Overview](docs/overview.md)
- [Setup](docs/setup.md)
- [Testing](docs/testing.md)
- [CI/CD](docs/ci-cd.md)
- [Release](docs/release.md)

## Installation

Install with your package manager of choice:

```bash
yarn add @wetteyve/rusty
```

## Vite Configuration (Optional)

If you are using Vite, you need to exclude the package from the dependency optimization:

```typescript
// vite.config.js
import { defineConfig } from 'vite';

export default defineConfig(() => ({
  ...,
  optimizeDeps: {
    exclude: [ ...,'@wetteyve/rusty'],
  },
  ...
}));
```

## Node.js Server Runtime

When running in a Node.js server environment, no special configuration is required. Simply import the package and use it directly:

```typescript
import { getArrayLength } from '@wetteyve/rusty'

const arr = [{}, 1, 'hello', true]

console.log(getArrayLength(arr)) // → 4
```

## Client-Side Usage (WASM)

1. To use the package in the browser, follow the settings for your package manager [here](https://napi.rs/docs/concepts/webassembly#-yarn).
2. You'll need to load the WASM module asynchronously in the client:

### Example with react-router:

```typescript
import { type Route } from './+types/home';

const arr = [{}, 1, 'hello', true];

// Setup WASM in client browser
export const clientLoader = async ({ serverLoader }: Route.ClientLoaderArgs) => {
  const [serverData, wasm] = await Promise.all([
    serverLoader(),
    import('@wetteyve/rusty'),  // Load WASM asynchronously
  ]);
  return { ...serverData, wasm };
};
clientLoader.hydrate = true;

export const HydrateFallback = () => <p>Loading...</p>;

const App = ({ loaderData: { wasm } }: Route.ComponentProps) => {
  const handleClick = () => window.alert(`WASM array length: ${wasm.getArrayLength(arr)}`);

  return (
    <div>
      <button onClick={handleClick}>PUSH FOR WASM!</button>
    </div>
  );
};

export default App;
```

This setup ensures the WASM functionality is properly loaded and accessible on the client-side. Adjust paths and logic according to your app structure.
