#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![allow(non_snake_case)]

use pixels::Error;

mod core {
    pub mod chip8;
    pub mod renderer;
    pub mod keyboard;
    pub mod speaker;
    pub mod cpu;
}

fn main() -> Result<(), Error> {
    core::chip8::init()
}