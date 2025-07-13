import { Writable } from 'node:stream';
import { createVirtualTTY as createWasm, VirtualTtyWasm } from 'virtual-tty-wasm';

/**
 * Configuration options for creating a virtual TTY
 * @typedef {Object} TtyInitArg
 * @property {number} [width=20] - Terminal width in columns
 * @property {number} [height=6] - Terminal height in rows
 */
type TtyInitArg = {
  width?: number,
  height?: number,
}

class VirtualTtyWritableStream extends Writable {
  private writeMethod: (data: string) => void;

  constructor(writeMethod: (data: string) => void) {
    super({ encoding: 'utf8' });
    this.writeMethod = writeMethod;
  }

  _write(chunk: any, encoding: BufferEncoding, callback: (error?: Error | null) => void): void {
    try {
      const data = chunk.toString();
      this.writeMethod(data);
      callback();
    } catch (error) {
      callback(error instanceof Error ? error : new Error(String(error)));
    }
  }
}

class VirtualTtyStreams {
  private wasm: VirtualTtyWasm;
  public readonly stdout: Writable;
  public readonly stderr: Writable;

  constructor(wasm: VirtualTtyWasm) {
    this.wasm = wasm;
    this.stdout = new VirtualTtyWritableStream((data) => this.wasm.stdoutWrite(data));
    this.stderr = new VirtualTtyWritableStream((data) => this.wasm.stderrWrite(data));
  }

  getSnapshot(): string {
    return this.wasm.getSnapshot();
  }
}

/**
 * Creates a virtual TTY with Node.js streams interface
 * @param {TtyInitArg} config - Configuration options
 * @param {number} [config.width=20] - Terminal width in columns
 * @param {number} [config.height=6] - Terminal height in rows
 * @returns {VirtualTtyStreams} Object with stdout/stderr streams and getSnapshot method
 * @example
 * const tty = createVirtualTTY({ width: 80, height: 24 });
 * tty.stdout.write("Hello World\n");
 * tty.stderr.write("Error message\n");
 * const snapshot = tty.getSnapshot();
 */
export function createVirtualTTY({width = 20, height = 6}: TtyInitArg = {}): VirtualTtyStreams {
  const wasm = createWasm(width, height);
  return new VirtualTtyStreams(wasm);
}

export type { TtyInitArg };
