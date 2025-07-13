# virtual-tty-wasm

WebAssembly bindings for the [virtual-tty](https://github.com/xavescor/virtual-tty) library - virtual terminal emulation for testing terminal applications.

## Installation

```bash
npm install virtual-tty-wasm
```

## Usage

### Node.js (ES Modules)

```javascript
import { createVirtualTTY } from 'virtual-tty-wasm';

// Create a virtual terminal (80 columns, 24 rows)
const tty = createVirtualTTY(80, 24);

// Write to stdout
tty.stdoutWrite("Hello, World!\n");

// Write to stderr
tty.stderrWrite("Error message\n");

// Get current terminal state
const snapshot = tty.getSnapshot();
console.log(snapshot);
```

### Node.js (CommonJS)

```javascript
const { createVirtualTTY } = require('virtual-tty-wasm');

const tty = createVirtualTTY(80, 24);
tty.stdoutWrite("Hello from WASM!\n");
console.log(tty.getSnapshot());
```

### TypeScript

```typescript
import { VirtualTtyWasm, createVirtualTTY } from 'virtual-tty-wasm';

const tty: VirtualTtyWasm = createVirtualTTY(80, 24);
tty.stdoutWrite("Hello, TypeScript!\n");
const output: string = tty.getSnapshot();
```

## API

### `createVirtualTTY(width: number, height: number): VirtualTtyWasm`

Creates a new virtual terminal with specified dimensions.

### `VirtualTtyWasm`

#### Methods

- `stdoutWrite(data: string): void` - Write data to stdout
- `stderrWrite(data: string): void` - Write data to stderr  
- `getSnapshot(): string` - Get current terminal state as string

## Features

- **Pure Virtual Terminal**: No real PTY dependencies
- **ANSI Support**: Handles common terminal escape sequences
- **Cross-Platform**: Works in Node.js 16+ environments
- **TypeScript**: Full TypeScript definitions included
- **Deterministic**: Perfect for testing terminal applications

## Use Cases

- Testing CLI applications
- Terminal output validation
- ANSI escape sequence processing
- Terminal emulation in web applications

## License

MIT

## Repository

https://github.com/xavescor/virtual-tty