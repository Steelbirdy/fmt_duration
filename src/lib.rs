mod parsing;
use logos::Logos;
use parsing::Token;
use std::fmt::Write;

pub trait FmtDuration {
    type Error;

    fn format(&self, fmt: &str) -> Result<String, Self::Error>;
}

impl FmtDuration for core::time::Duration {
    type Error = std::fmt::Error;

    fn format(&self, fmt: &str) -> Result<String, Self::Error> {
        let mut buf = String::new();

        let mut lex = Token::lexer(fmt);

        while let Some(token) = lex.next() {
            match token {
                Token::Error => panic!(),
                Token::NoFormat => {
                    buf.push_str(lex.slice());
                    Ok(())
                }
                Token::Percent => {
                    buf.push('%');
                    Ok(())
                }
                Token::HoursPadded => write!(&mut buf, "{:02}", self.as_secs() / (60 * 60)),
                Token::MinutesPadded => write!(&mut buf, "{:02}", self.as_secs() / 60 % 60),
                Token::SecondsPadded => write!(&mut buf, "{:02}", self.as_secs() % 60),
                Token::MillisPadded => write!(&mut buf, "{:03}", self.as_millis() % 1000),
                Token::MicrosPadded => write!(&mut buf, "{:06}", self.as_micros() % 1_000_000),
                Token::NanosPadded => write!(&mut buf, "{:09}", self.as_nanos() % 1_000_000_000),
                Token::HoursUnpadded => write!(&mut buf, "{}", self.as_secs() / (60 * 60)),
                Token::MinutesUnpadded => write!(&mut buf, "{}", self.as_secs() / 60 % 60),
                Token::SecondsUnpadded => write!(&mut buf, "{}", self.as_secs() % 60),
                Token::MillisUnpadded => write!(&mut buf, "{}", self.as_millis() % 1000),
                Token::MicrosUnpadded => write!(&mut buf, "{}", self.as_micros() % 1_000_000),
                Token::NanosUnpadded => write!(&mut buf, "{}", self.as_nanos() % 1_000_000_000),
            }?;
        }

        Ok(buf)
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

    #[test_case(as_secs(11, 53, 20), 001203885, "%H:%M:%S.%NS"     => String::from("11:53:20.001203885"))]
    #[test_case(as_secs(00, 00, 05), 000001000, "%H:%M:%S.%US"     => String::from("00:00:05.000001")   )]
    #[test_case(as_secs(01, 01, 01), 000000000, "%s.%MS/100%%"     => String::from("1.000/100%")        )]
    #[test_case(as_secs(15, 04, 07), 123456789, "%Hhr %mmin %ssec" => String::from("15hr 4min 7sec")    )]
    fn format_std_duration(secs: u64, nanos: u32, fmt_str: &str) -> String {
        StdDuration::format(&StdDuration::new(secs, nanos), fmt_str).unwrap()
    }
}
