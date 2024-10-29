/// The components of a CHIP8 emulator.
pub struct Emulator {
    memory: [u8; 4096],
    display: Vec<Vec<u8>>, // Encoded as rows of bytes in which each bit is a pixel
    program_counter: u16,
    index_register: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; 16]
}

pub impl Emulator {
    fn new(screen_width: usize, screen_height: usize) -> Self {
        Emulator {
            memory: [0; 4096],
            display: vec![vec![0; screen_width]; screen_height], // Access with display[row][col]
            program_counter: 0x200, // Start at address 512 since early emulators were stored in RAM before that address
            index_register: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16]
        }
    }

    fn draw(&self) {
        // TODO: Implement drawing to screen
    }
}
