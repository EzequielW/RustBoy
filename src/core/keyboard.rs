use std::collections::HashMap;

use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use super::cpu::CPU;

pub(crate) struct Keyboard{
    keymap: HashMap<VirtualKeyCode, u8>,
    keypressed: HashMap<u8, bool>
}

impl Keyboard {
    pub fn new() -> Keyboard {
        let keymap: HashMap<VirtualKeyCode, u8> = HashMap::from([
            (VirtualKeyCode::Key1, 0x1),
            (VirtualKeyCode::Key2, 0x2),
            (VirtualKeyCode::Key3, 0x3),
            (VirtualKeyCode::Key4, 0xC),
            (VirtualKeyCode::Q, 0x4),
            (VirtualKeyCode::W, 0x5),
            (VirtualKeyCode::E, 0x6),
            (VirtualKeyCode::R, 0xD),
            (VirtualKeyCode::A, 0x7),
            (VirtualKeyCode::S, 0x8),
            (VirtualKeyCode::D, 0x9),
            (VirtualKeyCode::F, 0xE),
            (VirtualKeyCode::Z, 0xA),
            (VirtualKeyCode::X, 0x0),
            (VirtualKeyCode::C, 0xB),
            (VirtualKeyCode::V, 0xF)
        ]);
        let keypressed: HashMap<u8, bool> = HashMap::from([
            (0x1, false),
            (0x2, false),
            (0x3, false),
            (0xC, false),
            (0x4, false),
            (0x5, false),
            (0x6, false),
            (0xD, false),
            (0x7, false),
            (0x8, false),
            (0x9, false),
            (0xE, false),
            (0xA, false),
            (0x0, false),
            (0xB, false),
            (0xF, false)
        ]);

        Keyboard { 
            keymap, 
            keypressed
        }
    }

    pub fn isKeyPressed(&self, keyCode: u8) -> bool {
        self.keypressed[&keyCode]
    }

    pub fn onKeyDown(&mut self, keyCode: u8, cpu: &mut CPU) {
        self.keypressed.insert(keyCode,  true);

        if cpu.isPaused() {
            cpu.unpause(keyCode);
        }
    }

    pub fn onKeyUp(&mut self, keyCode: u8) {
        self.keypressed.insert(keyCode, false);
    }

    pub fn handleInput(&mut self, input: &WinitInputHelper, cpu: &mut CPU) {
        for (key, value) in self.keymap.clone() {
            if input.key_pressed(key) {
                self.onKeyDown(value, cpu);
            }
            if input.key_released(key) {
                self.onKeyUp(value);
            }
        }
    }
}
