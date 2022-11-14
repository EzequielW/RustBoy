use chrono::Utc;
use pixels::Error;
use winit::{
    event_loop::{ControlFlow, EventLoop}, event::VirtualKeyCode, dpi::LogicalSize, window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use super::{renderer::{Renderer}, speaker::Speaker, cpu::CPU};
use super::keyboard::Keyboard;

pub(crate) fn init() -> Result<(), Error>{
    env_logger::init();
    let defaultWidth: usize = 64;
    let defaultHeight: usize = 32;

    let eventLoop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let scale: usize = 10;
    let window = {
        let size = LogicalSize::new(defaultWidth as f64, defaultHeight as f64);
        let scaled_size = LogicalSize::new(defaultWidth as f64 * scale as f64, defaultHeight as f64 * scale as f64);
        WindowBuilder::new()
            .with_title("Chip 8 Emulator")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&eventLoop)
            .unwrap()
    };
    let fpsInterval = 1000 / 60;
    let mut previousTime = Utc::now().time();

    // Main components
    let mut winRenderer = Renderer::new(&window)?;
    let mut keyboard = Keyboard::new();
    let mut speaker = Speaker::new();
    let mut cpu = CPU::new();
    let romName = String::from("PONG");
    cpu.loadSpritesIntoMemory();
    cpu.loadRom(&romName);

    eventLoop.run(move |event, _, controlFlow| {
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *controlFlow = ControlFlow::Exit;
                return;
            }

            keyboard.handleInput(&input, &mut cpu);
        }

        let currentTime = Utc::now().time();
        let elapsed = currentTime - previousTime;

        if elapsed.num_milliseconds() > fpsInterval {
            previousTime = currentTime;
            cpu.cycle(&mut winRenderer, &mut speaker, &mut keyboard);
        }
    });
}