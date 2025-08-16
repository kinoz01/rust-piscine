use std::env;

pub fn brainfuck(s: &str) {
    let c: Vec<u8> = s.bytes().filter(|b| b"< >+-[].".contains(b)).collect();
    let (mut t, mut p, mut pc) = ([0u8; 2048], 0usize, 0usize);
    while pc < c.len() {
        match c[pc] {
            b'>' => p = (p + 1) % 2048,
            b'<' => p = (p + 2047) % 2048,
            b'+' => t[p] = t[p].wrapping_add(1),
            b'-' => t[p] = t[p].wrapping_sub(1),
            b'.' => print!("{}", t[p] as char),
            b'[' => if t[p] == 0 {
                let (mut d, mut i) = (1, pc + 1);
                while d > 0 { d += (c[i] == b'[') as i32 - (c[i] == b']') as i32; i += 1; }
                pc = i - 1;
            },
            b']' => if t[p] != 0 {
                let (mut d, mut i) = (1, pc - 1);
                while d > 0 { d += (c[i] == b']') as i32 - (c[i] == b'[') as i32; i -= 1; }
                pc = i + 1;
            },
            _ => {}
        }
        pc += 1;
    }
}

fn main() {
    if let Some(src) = env::args().nth(1) {
        brainfuck(&src);
    }
}
