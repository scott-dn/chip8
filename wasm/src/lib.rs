use core::Emu;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct WasmEmu {
    emu: Emu,
}

#[wasm_bindgen]
impl WasmEmu {
    #[wasm_bindgen(constructor)]
    pub fn default() -> Self {
        Self {
            emu: Emu::default(),
        }
    }
}
