use crossterm::{
    event::{Event, KeyCode, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn read_char() -> u8 {
    loop {
        if let Ok(Event::Key(k)) = read() {
            match k.code {
                KeyCode::Char(c) => {
                    if let Some(ascii) = c.as_ascii() {
                        return ascii.to_u8();
                    } else {
                        return 255;
                    }
                }
                _ => return 255,
            }
        }
    }
}

fn run_program(program: &[u8], memory: &mut Vec<u8>, index: &mut usize) -> usize {
    let mut i = 0;
    while i < program.len() {
        let c = program[i];
        match c {
            b'>' => {
                *index += 1;
                while *index >= memory.len() {
                    memory.push(0);
                }
            }
            b'<' => {
                if *index > 0 {
                    *index -= 1;
                }
            }
            b'+' => {
                memory[*index] = memory[*index].wrapping_add(1);
            }
            b'-' => {
                memory[*index] = memory[*index].wrapping_sub(1);
            }
            b'[' => {
                while memory[*index] != 0 {
                    run_program(&program[i + 1..], memory, index);
                }
                loop {
                    i += 1;
                    if program[i] == b']' {
                        break;
                    }
                }
            }
            b']' => return i,
            b'.' => print!("{}", memory[*index] as char),
            b',' => memory[*index] = read_char(),

            _ => (),
        }
        i += 1;
    }
    return 0;
}

pub fn interpreter(program: &[u8]) {
    enable_raw_mode().unwrap();

    let mut mem = vec![0];
    let mut index = 0;

    run_program(program, &mut mem, &mut index);

    disable_raw_mode().unwrap();
}
