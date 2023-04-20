pub const TRACE: LogTag = LogTag(0b00000000_00000000_00000000_00000001);
pub const DEBUG: LogTag = LogTag(0b00000000_00000000_00000000_00000010);
pub const INFO: LogTag =  LogTag(0b00000000_00000000_00000000_00000100);
pub const WARN: LogTag =  LogTag(0b00000000_00000000_00000000_00001000);
pub const ERROR: LogTag =  LogTag(0b00000000_00000000_00000000_00010000);
pub const FATAL: LogTag =  LogTag(0b00000000_00000000_00000000_00100000);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogTag(u32);
impl BitOr for LogTag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
pub struct LogMask(u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
