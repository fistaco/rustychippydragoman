use std::time::{Instant, SystemTime};

/// The components of a CHIP8 emulator.
pub struct Emulator {
    memory: [u8; 4096],
    display: Vec<Vec<u8>>, // Encoded as rows of bytes in which each bit is a pixel
    program_counter: u16,
    index_register: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; 16],
    instructions_per_second: u32
}

impl Emulator {
    /// Initialises and returns a new CHIP-8 interpreter object.
    ///
    /// ## Arguments
    /// * `screen_width` - The width of the screen in pixels.
    /// * `screen_height` - The height of the screen in pixels.
    /// * `instructions_per_second` - The number of instructions the interpreter will process per second.
    fn new(screen_width: usize, screen_height: usize, instructions_per_second: u32) -> Self {
        Emulator {
            memory: [0; 4096],
            display: vec![vec![0; screen_width]; screen_height], // Access with display[row][col]
            program_counter: 0x200, // Start at address 512 since early emulators were stored in RAM before that address
            index_register: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16],
            instructions_per_second: instructions_per_second
        }
    }

    fn run(&mut self, refresh_rate: u8) {
        let instruction_interval_micros: u128 = 1e6 as u128 / self.instructions_per_second as u128;

        let mut prev_time = Instant::now();
        loop {
            let elapsed = prev_time.elapsed();

            if elapsed.as_micros() > instruction_interval_micros {
                prev_time = Instant::now();

                let instruction: u8 = self.fetch_instruction();
                self.execute_instruction(instruction);
            }
        }
    }

    /// Fetch the instruction at the current program counter
    fn fetch_instruction(&self) -> u8 {
        self.memory[self.program_counter as usize]
    }

    fn decode_instruction(&self, instruction: u8) -> u8 {
        0
    }

    fn execute_instruction(&mut self, instruction: u8) {
        // TODO
    }

    fn draw(&self) {
        // TODO: Implement drawing to screen
    }

    fn update_state(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            self.beep();
        }
    }

    fn beep(&self) {
        // TODO: Implement sound output
    }

    fn clear_screen(&mut self) {
        self.display.iter_mut().for_each(| display_columm: &mut Vec<u8> | display_columm.clear());
    }

    fn jump(&mut self, address: u16) {
        self.program_counter = address;
    }
}
