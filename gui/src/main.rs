use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::{fs::File, io::Read};

use std::env;

const SCALE: u32 = 15;
const TICKS_PER_FRAME: usize = 10;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    args.len().ge(&2).then_some(()).expect("ROM not found.");

    let mut buf = Vec::new();
    File::open(&args[1])
        .expect("Unable to open file")
        .read_to_end(&mut buf)
        .unwrap();

    let mut emu = core::Emu::default();
    emu.load(&buf);

    let sdl_context = sdl2::init().unwrap();
    let mut canvas = sdl_context
        .video()
        .unwrap()
        .window(
            "Chip8",
            core::SCREEN_W as u32 * SCALE,
            core::SCREEN_H as u32 * SCALE,
        )
        .position_centered()
        .opengl()
        .build()
        .unwrap()
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    let mut event = sdl_context.event_pump().unwrap();
    'game: loop {
        for e in event.poll_iter() {
            match e {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'game;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(idx) = map_key(keycode) {
                        emu.key_press(idx, true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(idx) = map_key(keycode) {
                        emu.key_press(idx, false);
                    }
                }
                _ => (),
            }
        }

        for _ in 0..TICKS_PER_FRAME {
            emu.exec();
        }
        emu.tick_timers();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for (i, &pixel) in emu.display().iter().enumerate() {
            if pixel {
                // 1D -> 2D
                let (x, y) = ((i % core::SCREEN_W) as u32, (i / core::SCREEN_W) as u32);
                canvas
                    .fill_rect(Rect::new(
                        (x * SCALE) as i32,
                        (y * SCALE) as i32,
                        SCALE,
                        SCALE,
                    ))
                    .unwrap();
            }
        }
        canvas.present();
    }
}

/*
    Keyboard                    Chip-8
    +---+---+---+---+           +---+---+---+---+
    | 1 | 2 | 3 | 4 |           | 1 | 2 | 3 | C |
    +---+---+---+---+           +---+---+---+---+
    | Q | W | E | R |           | 4 | 5 | 6 | D |
    +---+---+---+---+     =>    +---+---+---+---+
    | A | S | D | F |           | 7 | 8 | 9 | E |
    +---+---+---+---+           +---+---+---+---+
    | Z | X | C | V |           | A | 0 | B | F |
    +---+---+---+---+           +---+---+---+---+
*/

fn map_key(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}
