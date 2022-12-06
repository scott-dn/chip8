use core::{Emu, SCREEN_W};
use js_sys::Uint8Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

#[wasm_bindgen]
pub struct WasmEmu {
    emu: Emu,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl WasmEmu {
    #[wasm_bindgen(constructor)]
    pub fn default() -> Self {
        let doc = window().unwrap().document().unwrap();
        let canvas = doc
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        Self {
            emu: Emu::default(),
            ctx,
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.emu.reset();
    }

    #[wasm_bindgen]
    pub fn exec(&mut self) {
        self.emu.exec();
    }

    #[wasm_bindgen]
    pub fn tick_timers(&mut self) {
        self.emu.tick_timers();
    }

    #[wasm_bindgen]
    pub fn key_press(&mut self, event: KeyboardEvent, pressed: bool) {
        if let Some(k) = key2btn(&event.key()) {
            self.emu.key_press(k, pressed);
        }
    }

    #[wasm_bindgen]
    pub fn load(&mut self, data: Uint8Array) {
        self.emu.load(&data.to_vec());
    }

    #[wasm_bindgen]
    pub fn draw_screen(&self, scale: usize) {
        for (i, &pixel) in self.emu.display().iter().enumerate() {
            if pixel {
                let (x, y) = (i % SCREEN_W, i / SCREEN_W);
                self.ctx.fill_rect(
                    (x * scale) as f64,
                    (y * scale) as f64,
                    scale as f64,
                    scale as f64,
                );
            }
        }
    }
}

fn key2btn(key: &str) -> Option<usize> {
    match key {
        "1" => Some(0x1),
        "2" => Some(0x2),
        "3" => Some(0x3),
        "4" => Some(0xC),
        "q" => Some(0x4),
        "w" => Some(0x5),
        "e" => Some(0x6),
        "r" => Some(0xD),
        "a" => Some(0x7),
        "s" => Some(0x8),
        "d" => Some(0x9),
        "f" => Some(0xE),
        "z" => Some(0xA),
        "x" => Some(0x0),
        "c" => Some(0xB),
        "v" => Some(0xF),
        _ => None,
    }
}
