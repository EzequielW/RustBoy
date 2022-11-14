use std::{fs::read};

use rand::Rng;

use super::{renderer::Renderer, speaker::Speaker, keyboard::Keyboard};

pub(crate) struct CPU{
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    delayTimer: u8,
    soundTimer: u8,
    pc: u16,
    stack: Vec<u16>,
    paused: bool,
    speed: usize,
    unpauseVIndex: usize 
}

impl CPU {
    pub fn new() -> CPU {
        let memory: [u8; 4096] = [0; 4096];
        let v: [u8; 16] = [0; 16];
        let i: u16 = 0;
        let delayTimer: u8 = 0;
        let soundTimer: u8 = 0;
        let pc: u16 = 0x200;
        let stack = vec![0;0];
        let paused = false;
        let speed: usize = 10;

        CPU {
            memory,
            v,
            i,
            delayTimer,
            soundTimer,
            pc,
            stack,
            paused,
            speed,
            unpauseVIndex: 0
        }
    }

    pub fn loadSpritesIntoMemory(&mut self){
        let sprites: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        for i in 0..sprites.len() {
            self.memory[i] = sprites[i];
        }
    }

    pub fn loadProgramIntoMemory(&mut self, program: Vec<u8>){
        let programStart = 0x200;

        for i in 0..program.len() {
            self.memory[programStart + i] = program[i];
        }
    }

    pub fn loadRom(&mut self, romName: &String){
        let mut path: String = "./src/roms/".to_owned();
        path.push_str(romName);
        let program = read(path).unwrap();

        self.loadProgramIntoMemory(program);
    }

    pub fn isPaused(&self) -> bool{
        self.paused
    }

    pub fn unpause(&mut self, keyCode: u8){
        self.paused = false;
        self.v[self.unpauseVIndex] = keyCode;
    }

    pub fn cycle(&mut self, renderer: &mut Renderer, speaker: &mut Speaker, keyboard: &mut Keyboard){
        for _ in 0..self.speed {
            if !self.paused {
                let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;
                self.executeInstruction(opcode, renderer, keyboard);
            }
        }

        if !self.paused {
            self.updateTimers();
        }

        self.playSound(speaker);
        renderer.render();
    }

    pub fn updateTimers(&mut self) {
        if self.delayTimer > 0 {
            self.delayTimer -= 1;
        }

        if self.soundTimer > 0 {
            self.soundTimer -= 1;
        }
    }

    pub fn playSound(&mut self, speaker: &mut Speaker) {
        if self.soundTimer > 0 {
            speaker.play(Some(440.0));
        } else {
            speaker.stop();
        }
    }

    pub fn executeInstruction(&mut self, opcode: u16, renderer: &mut Renderer, keyboard: &mut Keyboard) {
        self.pc += 2;

        let x: u8 = ((opcode & 0x0F00) >> 8) as u8;
        let y: u8 = ((opcode & 0x00F0) >> 4) as u8;

        match opcode & 0xF000 {
            0x0000 => {
                match opcode {
                    0x00E0 => renderer.clear(),
                    0x00EE => self.pc = self.stack.pop().unwrap(),
                    _ => println!("Unknown 0 opcode {}", opcode)
                }
            },
            0x1000 => self.pc = opcode & 0xFFF,
            0x2000 => {
                self.stack.push(self.pc);
                self.pc = opcode & 0xFFF;
            },
            0x3000 => {
                if self.v[x as usize] == (opcode & 0xFF) as u8 {
                    self.pc += 2;
                }
            },
            0x4000 => {
                if self.v[x as usize] != (opcode & 0xFF) as u8 {
                    self.pc += 2;
                }
            },
            0x5000 => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            },
            0x6000 => self.v[x as usize] = (opcode & 0xFF) as u8,
            0x7000 => self.v[x as usize] = self.v[x as usize].wrapping_add((opcode & 0xFF) as u8),
            0x8000 => {
                match opcode & 0xF {
                    0x0 => self.v[x as usize] = self.v[y as usize],
                    0x1 => self.v[x as usize] |= self.v[y as usize],
                    0x2 => self.v[x as usize] &= self.v[y as usize],
                    0x3 => self.v[x as usize] ^= self.v[y as usize],
                    0x4 => {
                        let sum: u16 = self.v[x as usize] as u16 + self.v[y as usize] as u16;

                        self.v[0xF] = 0;
                        if sum > 0xFF {
                            self.v[0xF] = 1;
                        }

                        self.v[x as usize] = (sum & 0xFFFF) as u8;
                    },
                    0x5 => {
                        self.v[0xF] = 0;
                        if self.v[x as usize] > self.v[y as usize] {
                            self.v[0xF] = 1;
                        }

                        self.v[x as usize] = self.v[x as usize].wrapping_sub(self.v[y as usize]);
                    },
                    0x6 => {
                        self.v[0xF] = self.v[x as usize] & 0x1;
                        self.v[x as usize] >>= 1;
                    },
                    0x7 => {
                        self.v[0xF] = 0;
                        if self.v[y as usize] > self.v[x as usize] {
                            self.v[0xF] = 1;
                        }

                        self.v[x as usize] = self.v[y as usize] - self.v[x as usize];
                    },
                    0xE => {
                        self.v[0xF] = self.v[x as usize] & 0x80;
                        self.v[x as usize] <<= 1;
                    },
                    _ => println!("Unknown 8 opcode {}", opcode & 0xF)
                }
            },
            0x9000 => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            },
            0xA000 => self.i = opcode & 0xFFF,
            0xB000 => self.pc = opcode & 0xFFF + self.v[0] as u16,
            0xC000 => {
                let mut rng = rand::thread_rng();
                let randomNum:u8 = rng.gen_range(0..0xFF);

                self.v[x as usize] = randomNum & opcode as u8;
            },
            0xD000 => {
                let width = 8;
                let height = opcode & 0xF;

                self.v[0xF] = 0;

                for row in 0..height {
                    let mut sprite = self.memory[(self.i + row) as usize];
                    
                    for col in 0..width {
                        if (sprite & 0x80) > 0 {
                            let pixelX = (self.v[x as usize] as u16 + col) as usize;
                            let pixelY = (self.v[y as usize] as u16 + row) as usize;
                            if renderer.setPixel(pixelX, pixelY) {
                                self.v[0xF] = 1;
                            }
                        }

                        sprite <<= 1;
                    }
                }
            },
            0xE000 => {
                match opcode & 0xFF {
                    0x9E => {
                        if keyboard.isKeyPressed(self.v[x as usize]) {
                            self.pc += 2;
                        }
                    },
                    0xA1 => {
                        if !keyboard.isKeyPressed(self.v[x as usize]) {
                            self.pc += 2;
                        }
                    },
                    _ => println!("Unknown E opcode {}", opcode & 0xFF)
                }
            },
            0xF000 => {
                match opcode & 0xFF {
                    0x07 => self.v[x as usize] = self.delayTimer,
                    0x0A => {
                        self.paused = true;
                        self.unpauseVIndex = x as usize;
                    },
                    0x15 => self.delayTimer = self.v[x as usize],
                    0x18 => self.soundTimer = self.v[x as usize],
                    0x1E => self.i += self.v[x as usize] as u16,
                    0x29 => self.i = self.v[x as usize] as u16 * 5,
                    0x33 => {
                        // Get the hundreds digit and place it in I.
                        self.memory[self.i as usize] = self.v[x as usize] / 100;

                        // Get tens digit and place it in I+1. Gets a value between 0 and 99,
                        // then divides by 10 to give us a value between 0 and 9.
                        self.memory[self.i as usize + 1] = (self.v[x as usize] % 100) / 10;

                        // Get the value of the ones (last) digit and place it in I+2.
                        self.memory[self.i as usize + 2] = self.v[x as usize] % 10;
                    },
                    0x55 => {
                        for registerIndex in 0..(x + 1) {
                            self.memory[(self.i + registerIndex as u16) as usize] = self.v[registerIndex as usize];
                        }
                    },
                    0x65 => {
                        for registerIndex in 0..(x + 1) {
                            self.v[registerIndex as usize] = self.memory[(self.i + registerIndex as u16) as usize];
                        }
                    },
                    _ => println!("Unknown F opcode {}", opcode & 0xFF)
                }
            },
            _ => println!("Unknown opcode {}", opcode)
        }
    }
}