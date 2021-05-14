mod parsing;

pub trait FmtDuration {}

impl FmtDuration for core::time::Duration {}
impl FmtDuration for chrono::Duration {}

#[cfg(test)]
mod tests {
    #[test]
    fn sanity_check() {
        assert_eq!(2 + 2, 4);
    }
}
