use super::common::initial_offset;
use crate::{golem, ingredients, CodeAssignments};

const NEW_5C_OFFSETS: [i32; 2] = [0x6, 0x4];

pub fn parse_code(code: &[char]) -> CodeAssignments {
    let cchar4 = code[4] as i32;
    let cchar3 = code[3] as i32;
    let mut cval2 = (code[2] as i32) - 0x41;
    let cval1 = (code[1] as i32) - 0x41;
    let cval0 = (code[0] as i32) - 0x41;

    let cval_mod2s = [
        (cval0 - (cval0 >> 0x1f) & 1) + (cval0 >> 0x1f),
        (cval1 - (cval1 >> 0x1f) & 1) + (cval1 >> 0x1f),
        (cval2 - (cval2 >> 0x1f) & 1) + (cval2 >> 0x1f),
    ];

    let mut vals = vec![0u8; 7];
    let mut alloced2 = vec![0u8; 2];
    // let initial_offset =
    // NEW_OFFSETS[(cval_mod2s[0] + cval_mod2s[2] * 4 + cval_mod2s[1] * 2) as usize];
    let initial_offset = initial_offset(code);
    let mut uvar13;
    let mut uvar14;

    let mut uvar8 = (initial_offset * -0x48b9
        + ((((cchar3 - 0x41) + (cchar4 - 0x41) * 0x1a) * 0xd + cval2 / 2) * 0xd + cval1 / 2) * 0xd
        + cval0 / 2
        + 0x16a974)
        % 0x16a974;

    let mut i = 0;

    while i != 6 {
        let uvar10 = i % 3;
        if cval_mod2s[uvar10] == (i as i32) / 3 {
            if uvar10 != 2 {
                cval2 = NEW_5C_OFFSETS[uvar10];
                uvar14 = uvar8 % cval2;
                uvar8 = uvar8 / cval2;
                alloced2[uvar10] = uvar14 as u8;
            }

            cval2 = uvar8 >> 0x1f;
            uvar13 = uvar8 - cval2;
            uvar8 = uvar8 / 2;

            vals[0] = vals[0] + (((uvar13 & 1) + cval2 << (uvar10 & 0xff)) as u8);
        }

        cval2 = uvar8 % (7 - i as i32);
        uvar8 = uvar8 / (7 - i as i32);

        vals[i + 1] = cval2 as u8;

        i += 1;
    }

    CodeAssignments {
        ingredients: ingredients::from_lehmer(&vals),
        golem: Some(golem::from_lehmer(&alloced2)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn confirm_result(code: &str, expected_ingredients: [&str; 8], expected_golem: &str) {
        let chars: Vec<char> = code.chars().collect();

        let result = parse_code(chars.as_slice());

        assert_eq!(
            result.ingredients.values().collect::<Vec<&'static str>>(),
            expected_ingredients
        );
        assert_eq!(
            result
                .golem
                .expect("expected golem results")
                .values()
                .collect::<Vec<_>>()
                .join(""),
            expected_golem
        );
    }

    // ingredient order: fern, claw, mushroom, flower, root, scorpion, toad, feather
    // golem order: head, chest

    // each of these tests are for a different value of cvals_mod_2
    #[test]
    fn code_ocegf() {
        confirm_result(
            "OCEGF",
            ["G-", "--", "++", "R+", "G+", "R-", "B+", "B-"],
            "rg",
        );
    }

    #[test]
    fn code_mudtc() {
        confirm_result(
            "MUDTC",
            ["++", "--", "G+", "B+", "B-", "G-", "R-", "R+"],
            "Rg",
        );
    }

    #[test]
    fn code_clgvd() {
        confirm_result(
            "CLGVD",
            ["G-", "++", "R-", "B-", "--", "B+", "R+", "G+"],
            "Gb",
        );
    }

    #[test]
    fn code_ivtii() {
        confirm_result(
            "IVTII",
            ["R+", "R-", "--", "B-", "G-", "G+", "B+", "++"],
            "RG",
        );
    }

    #[test]
    fn code_daard() {
        confirm_result(
            "DAARD",
            ["G-", "B-", "R+", "G+", "B+", "R-", "--", "++"],
            "gR",
        );
    }

    #[test]
    fn code_nchne() {
        confirm_result(
            "NCHNE",
            ["B-", "B+", "G+", "--", "G-", "++", "R-", "R+"],
            "BG",
        );
    }

    #[test]
    fn code_hnkub() {
        confirm_result(
            "HNKUB",
            ["G+", "B-", "--", "R-", "++", "R+", "B+", "G-"],
            "bg",
        );
    }

    #[test]
    fn code_rxtue() {
        confirm_result(
            "RXTUE",
            ["B+", "++", "R-", "B-", "--", "G+", "G-", "R+"],
            "rb",
        );
    }

    // this code returns different values on Android and online - we should return the Android value
    #[test]
    fn code_golem() {
        confirm_result(
            "GOLEM",
            ["R+", "++", "G-", "B+", "G+", "R-", "B-", "--"],
            "bG",
        );
    }
}
