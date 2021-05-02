use super::common::initial_offset;
use crate::{ingredients, CodeAssignments};

pub fn parse_code(code: &[char]) -> CodeAssignments {
    let cchar3 = code[3] as i32;
    let mut cval2 = code[2] as i32 - 0x41;
    let cval1 = code[1] as i32 - 0x41;
    let cval0 = code[0] as i32 - 0x41;

    let cvals_mod_2 = [
        (cval0 - (cval0 >> 0x1f) & 1) + (cval0 >> 0x1f),
        (cval1 - (cval1 >> 0x1f) & 1) + (cval1 >> 0x1f),
        (cval2 - (cval2 >> 0x1f) & 1) + (cval2 >> 0x1f),
    ];

    let ivar5 = initial_offset(code);

    let mut vals = vec![0u8; 7];

    let mut i = 0;

    let mut uvar6 = (ivar5 * -0x265
        + (((cchar3 - 0x46) * 0xd + cval2 / 2) * 0xd + cval1 / 2) * 0xd
        + cval0 / 2
        + 0xb439)
        % 0xb439;

    let mut uvar7;

    while i != 6 {
        if cvals_mod_2[i % 3] == (i as i32) / 3 {
            cval2 = uvar6 >> 0x1f;
            uvar7 = uvar6 - cval2;
            uvar6 = uvar6 / 2;

            vals[0] = vals[0] + ((uvar7 & 1) + cval2 << (i % 3 & 0xff)) as u8;
        }

        cval2 = uvar6 % (7 - i as i32);
        uvar6 = uvar6 / (7 - i as i32);

        vals[i + 1] = cval2 as u8;

        i += 1;
    }

    CodeAssignments {
        ingredients: ingredients::from_lehmer(&vals),
        golem: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn confirm_result(code: &str, expected: [&str; 8]) {
        let chars: Vec<char> = code.chars().collect();

        let result = parse_code(chars.as_slice());

        assert_eq!(
            result.ingredients.values().collect::<Vec<&'static str>>(),
            expected
        );
    }

    // ingredient order: fern, claw, mushroom, flower, root, scorpion, toad, feather

    // each of these tests are for a different value of cvals_mod_2
    #[test]
    fn code_qwin() {
        confirm_result("QWIN", ["R+", "G+", "B+", "B-", "R-", "--", "G-", "++"]);
    }

    #[test]
    fn code_cejt() {
        confirm_result("CEJT", ["B+", "G+", "R-", "B-", "++", "--", "R+", "G-"]);
    }

    #[test]
    fn code_uhip() {
        confirm_result("UHIP", ["++", "G+", "G-", "R+", "R-", "B-", "B+", "--"]);
    }

    #[test]
    fn code_sdvz() {
        confirm_result("SDVZ", ["--", "G+", "B-", "B+", "++", "R+", "R-", "G-"]);
    }

    #[test]
    fn code_hqmt() {
        confirm_result("HQMT", ["B+", "++", "G-", "R-", "B-", "R+", "--", "G+"]);
    }

    #[test]
    fn code_nmrx() {
        confirm_result("NMRX", ["++", "R+", "G+", "--", "R-", "G-", "B+", "B-"]);
    }

    #[test]
    fn code_dtqh() {
        confirm_result("DTQH", ["G+", "B+", "--", "G-", "R+", "++", "B-", "R-"]);
    }

    #[test]
    fn code_jfbl() {
        confirm_result("JFBL", ["B+", "B-", "++", "R-", "R+", "G+", "--", "G-"]);
    }
}
