use crate::{CodeAssignments, DecodeError};

/// Remaps any known values that have different results on the web app
pub fn match_web(code: &str, assignments: CodeAssignments) -> Result<CodeAssignments, DecodeError> {
    if code.len() != 5 {
        return Ok(assignments);
    }

    let fourth_char = code.chars().nth(3).unwrap().to_ascii_uppercase();

    if !('A'..='E').contains(&fourth_char) {
        return Ok(assignments);
    }

    // it's a five character code, the fourth letter is A-E, therefore it uses different ingredient assignments (as if
    // the game code was just the first four characters)

    let override_code: String = code.chars().take(4).collect();

    Ok(CodeAssignments {
        ingredients: crate::decode(&override_code)?.ingredients,
        golem: assignments.golem,
    })
}
