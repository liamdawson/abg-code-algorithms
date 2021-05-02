const EVEN_ODD_OFFSETS: [i32; 8] = [7, 13, 19, 29, 37, 43, 53, 61];

pub fn initial_offset(code: &[char]) -> i32 {
    if code.len() < 3 {
        panic!("code was too short");
    }

    let mods = [
        (code[0] as usize - 0x41) % 2,
        (code[1] as usize - 0x41) % 2,
        (code[2] as usize - 0x41) % 2,
    ];

    EVEN_ODD_OFFSETS[mods[0] + 2 * mods[1] + 4 * mods[2]]
}
