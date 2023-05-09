use std::{collections::{BTreeSet, BTreeMap}, sync::atomic::AtomicU32, fmt::Display, ops::{BitAnd, Deref, BitOr}};


pub const FATAL: u32 = 0b00000000_00000000_00000000_00000001;
pub const ERROR: u32 = 0b00000000_00000000_00000000_00000010;
pub const WARN: u32 = 0b00000000_00000000_00000000_00000100;
pub const INFO: u32 = 0b00000000_00000000_00000000_00001000;
pub const DEBUG: u32 = 0b00000000_00000000_00000000_00010000;
pub const TRACE: u32 = 0b00000000_00000000_00000000_00100000;
pub const LOG_NON: u32 = 0b00000000_00000000_00000000_00000000;



const UNINIT: usize = 0;
static STATUS: AtomicUsize = AtomicUsize::new(UNINIT);
static GLOBAL_LOGGER_ID: AtomicU32 = AtomicU32::new(0);

pub struct LogBitMask(u32);

impl Display for LogBitMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            TRACE => write!(f, "{}", "TRACE"),
            DEBUG => write!(f, "{}", "DEBUG"),
            INFO => write!(f, "{}", "INFO"),
            WARN => write!(f, "{}", "WARN"),
            ERROR => write!(f, "{}", "ERROR"),
            FATAL => write!(f, "{}", "FATAL"),
            _ => write!(f, "{:X}", self.0),
        }
    }
}
impl Deref for LogBitMask {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Log {
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogMask(u32)
impl LogMask {
    #[inline(always)]
    pub fn new(mask: u32) -> Self {
        LogMask(mask)
    }
    #[inline(always)]
    pub fn all() -> Self {
        LogMask(LOG_ALL)
    }
    #[inline(always)]
    pub fn non() -> Self {
        LogMask(LOG_NON)
    }
    #[inline(always)]
    pub fn except(mask: u32) -> Self {
        LogMask(mask ^ LOG_ALL)
    }
    #[inline(always)]
    pub fn besides(&mut self, mask: u32) {
        self.0 &= !mask;
    }
    #[inline(always)]
    pub fn add(&mut self, mask: u32) {
        self.0 |= mask;
    }
}
impl Default for LogMask {
    fn default() -> Self {
        // TODO Should debug build add INFO?
        LogMask(WARN | ERROR | FATAL)
    }
}

pub struct FilePermit {
    file: String,
    lines: Option<BTreeSet<u32>>,
}

pub struct TimePermit {
    time_start: u64,
    duration: Option<u64>,
}

pub struct LogInPermit {
    gate: LogMask,
    groups: Option<BTreeSet<String>>,
    files: Option<BTreeSet<FilePermit>>,
    modules: Option<BTreeSet<String>>,
    times: Option<BTreeSet<TimePermit>>,
}

pub struct LogOutPermit {
    gate: bool,
    group: bool,
    file: bool,
    module: bool,
    line: bool,
    time_stamp: bool,
    proc_id: bool,
    thread_id: bool,
}
pub struct LogMetadata {
    mask: LogMask,
    group: &'a str,
    file: &'static str,
    module: &'static str,
    line: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LoggerId {
    id: u32,
}

struct LoggerContext {
    logger: Box<dyn Log>,
    in_permit: LogInPermit,
    out_permit: LogOutPermit,
}

pub(crate) struct LoggerRepo {
    repo: std::sync::RwLock<BTreeMap<LoggerId, LogContext>>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
