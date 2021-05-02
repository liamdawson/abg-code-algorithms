use crate::{ingredients, CodeAssignments};

pub fn parse_code(code: &[char]) -> CodeAssignments {
    let mut vals = Vec::new();

    let mut remaining_lehmer_choices = 8u8;
    let char_vals: Vec<_> = code.iter().map(|c| (*c as u8) - ('A' as u8)).collect();

    while remaining_lehmer_choices > 1 {
        let char_index = 3 - ((remaining_lehmer_choices as i32) - 5).abs() as usize;
        let mut char_value = char_vals[char_index];

        if 3 < 8 - remaining_lehmer_choices {
            char_value = char_value / (10 - remaining_lehmer_choices);
        }

        let val = char_value % remaining_lehmer_choices;

        vals.push(val);

        remaining_lehmer_choices -= 1;
    }

    CodeAssignments {
        ingredients: ingredients::from_lehmer(&vals),
        golem: None,
    }
}
