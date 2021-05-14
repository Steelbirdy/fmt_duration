use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
pub enum Token {
    #[token("%H")]
    HoursPadded,

    #[token("%h")]
    HoursUnpadded,

    #[token("%M")]
    MinutesPadded,

    #[token("%m")]
    MinutesUnpadded,

    #[token("%S")]
    SecondsPadded,

    #[token("%s")]
    SecondsUnpadded,

    #[token("%MS")]
    MillisPadded,

    #[token("%ms")]
    MillisUnpadded,

    #[token("%US")]
    MicrosPadded,

    #[token("%us")]
    MicrosUnpadded,

    #[token("%NS")]
    NanosPadded,

    #[token("%ns")]
    NanosUnpadded,

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

    #[test_case("%H"  => vec![(HoursPadded,     0..2)] ; "Lex padded hours"         )]
    #[test_case("%M"  => vec![(MinutesPadded,   0..2)] ; "Lex padded minutes"       )]
    #[test_case("%S"  => vec![(SecondsPadded,   0..2)] ; "Lex padded seconds"       )]
    #[test_case("%MS" => vec![(MillisPadded,    0..3)] ; "Lex padded milliseconds"  )]
    #[test_case("%US" => vec![(MicrosPadded,    0..3)] ; "Lex padded microseconds"  )]
    #[test_case("%NS" => vec![(NanosPadded,     0..3)] ; "Lex padded nanoseconds"   )]
    #[test_case("%h"  => vec![(HoursUnpadded,   0..2)] ; "Lex unpadded hours"       )]
    #[test_case("%m"  => vec![(MinutesUnpadded, 0..2)] ; "Lex unpadded minutes"     )]
    #[test_case("%s"  => vec![(SecondsUnpadded, 0..2)] ; "Lex unpadded seconds"     )]
    #[test_case("%ms" => vec![(MillisUnpadded,  0..3)] ; "Lex unpadded milliseconds")]
    #[test_case("%us" => vec![(MicrosUnpadded,  0..3)] ; "Lex unpadded microseconds")]
    #[test_case("%ns" => vec![(NanosUnpadded,   0..3)] ; "Lex unpadded nanoseconds" )]
    #[test_case("%%"  => vec![(Percent,         0..2)] ; "Lex escaped percent"      )]
    fn lex_tokens(input: &str) -> Vec<(Token, Range<usize>)> {
        get_spanned(input)
    }

    #[test]
    fn lex_hms_format() {
        let input = "%H:%M:%S.%MS";

        let expected = vec![
            (HoursPadded, 0..2),
            (NoFormat, 2..3),
            (MinutesPadded, 3..5),
            (NoFormat, 5..6),
            (SecondsPadded, 6..8),
            (NoFormat, 8..9),
            (MillisPadded, 9..12),
        ];

        assert_eq!(get_spanned(input), expected);
    }
}
