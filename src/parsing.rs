use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
pub enum Token {
    #[token("%H")]
    Hours,

    #[token("%M")]
    Minutes,

    #[token("%S")]
    Seconds,

    #[token("%MS")]
    Millis,

    #[token("%US")]
    Micros,

    #[token("%NS")]
    Nanos,

    #[token("%%")]
    Percent,

    #[regex("[^%]+")]
    NoFormat,

    #[error]
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range;
    use test_case::test_case;

    use super::Token::*;

    fn get_spanned(input: &str) -> Vec<(Token, Range<usize>)> {
        Token::lexer(input).spanned().collect::<Vec<_>>()
    }

    #[test_case("%H"  => vec![(Hours,   0..2)] ; "Lex hours"          )]
    #[test_case("%M"  => vec![(Minutes, 0..2)] ; "Lex minutes"        )]
    #[test_case("%S"  => vec![(Seconds, 0..2)] ; "Lex seconds"        )]
    #[test_case("%MS" => vec![(Millis,  0..3)] ; "Lex milliseconds"   )]
    #[test_case("%US" => vec![(Micros,  0..3)] ; "Lex microseconds"   )]
    #[test_case("%NS" => vec![(Nanos,   0..3)] ; "Lex nanoseconds"    )]
    #[test_case("%%"  => vec![(Percent, 0..2)] ; "Lex escaped percent")]
    fn lex_tokens(input: &str) -> Vec<(Token, Range<usize>)> {
        get_spanned(input)
    }

    #[test]
    fn lex_hms_format() {
        let input = "%H:%M:%S.%MS";

        let expected = vec![
            (Hours, 0..2),
            (NoFormat, 2..3),
            (Minutes, 3..5),
            (NoFormat, 5..6),
            (Seconds, 6..8),
            (NoFormat, 8..9),
            (Millis, 9..12),
        ];

        assert_eq!(get_spanned(input), expected);
    }
}
