use super::CodeAssignments;

mod common;
mod v1;
mod v2;
mod v3;

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("code is too short, should be at least 4 letters")]
    TooShort,
    #[error("code is too short, should be at most 5 letters")]
    TooLong,
    #[error("code should only be ascii alphabetical letters")]
    NonAsciiAlphabetic,
}

pub fn decode(input_code: &str) -> Result<CodeAssignments, DecodeError> {
    // choice rules mostly sourced from:
    // https://boardgamegeek.com/thread/1333522/article/18913539#18913539

    let code: Vec<char> = input_code.to_ascii_uppercase().chars().collect();

    // only 4 or 5 character ascii-alphabetic codes are supported
    if code.len() < 4 {
        return Err(DecodeError::TooShort);
    } else if code.len() > 5 {
        return Err(DecodeError::TooLong);
    } else if code.iter().any(|c| !c.is_ascii_alphabetic()) {
        return Err(DecodeError::NonAsciiAlphabetic);
    }

    // DEMO is special-cased to v1 to keep the rulebook accurate
    if code == ['D', 'E', 'M', 'O'] {
        return Ok(v1::parse_code(code.as_slice()));
    }

    // 5-character codes are always v3
    if code.len() == 5 {
        return Ok(v3::parse_code(code.as_slice()));
    }

    // if the last (4th) character is A, B, C, D or E, use v1
    // otherwise, use v2
    Ok(match code[3] {
        'A'..='E' => v1::parse_code(code.as_slice()),
        _ => v2::parse_code(code.as_slice()),
    })
}
