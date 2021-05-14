mod parsing;
use parsing::Token;
use logos::Logos;

pub trait FmtDuration {
    fn format(&self, fmt: &str) -> String;
}

impl FmtDuration for core::time::Duration {
    fn format(&self, fmt: &str) -> String {
        let mut buf = String::new();

        let mut lex = Token::lexer(fmt);

        while let Some(token) = lex.next() {
            match token {
                Token::Error => panic!("Unexpected token: '{}'", lex.slice()),
                Token::NoFormat => buf.push_str(lex.slice()),
                Token::Percent => buf.push('%'),
                Token::Hours => buf = format!("{}{}", buf, self.as_secs() / (60 * 60)),
                Token::Minutes => buf = format!("{}{:02}", buf, (self.as_secs() / 60) % 60),
                Token::Seconds => buf = format!("{}{:02}", buf, self.as_secs() % 60),
                Token::Nanos => buf = format!("{}{:09}", buf, self.as_nanos() % 1_000_000_000),
            }
        }

        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::FmtDuration;
    use core::time::Duration as StdDuration;
    use test_case::test_case;

    fn as_secs(hrs: u64, mins: u64, secs: u64) -> u64 {
        60 * 60 * hrs + 60 * mins + secs
    }

    #[test]
    fn sanity_check() {
        assert_eq!(2 + 2, 4);
    }

    #[test_case(as_secs(11, 53, 20), 001203885, "%H:%M:%S.%N" => String::from("11:53:20.001203885"))]
    #[test_case(as_secs(00, 00, 05), 000001000, "%H:%M:%S.%N" => String::from("0:00:05.000001000"))]
    #[test_case(as_secs(1,  01, 01), 000000000, "%S/100%%"    => String::from("01/100%"))]
    fn format_std_duration(secs: u64, nanos: u32, fmt_str: &str) -> String {
        StdDuration::format(&StdDuration::new(secs, nanos), fmt_str)
    }
}
