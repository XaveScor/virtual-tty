use std::io::Write;
use wasm_bindgen::prelude::*;
use virtual_tty::{VirtualTty, VirtualTtyStreams};

#[wasm_bindgen]
pub struct VirtualTtyWasm {
    streams: VirtualTtyStreams,
}

#[wasm_bindgen]
impl VirtualTtyWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> VirtualTtyWasm {
        VirtualTtyWasm {
            streams: VirtualTty::new(width, height),
        }
    }

    #[wasm_bindgen(js_name = stdoutWrite)]
    pub fn stdout_write(&mut self, data: &str) {
        let _ = write!(&mut self.streams.stdout, "{}", data);
    }

    #[wasm_bindgen(js_name = stderrWrite)]
    pub fn stderr_write(&mut self, data: &str) {
        let _ = write!(&mut self.streams.stderr, "{}", data);
    }

    #[wasm_bindgen(js_name = getSnapshot)]
    pub fn get_snapshot(&self) -> String {
        self.streams.get_snapshot()
    }
}

#[wasm_bindgen(js_name = createVirtualTTY)]
pub fn create_virtual_tty(width: usize, height: usize) -> VirtualTtyWasm {
    VirtualTtyWasm::new(width, height)
}