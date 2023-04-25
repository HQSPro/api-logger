
mod log_impl;
pub use log_impl::{LogInPermit, LogOutPermit, Log, LoggerId};

pub fn register_logger(logger: Box<dyn Log>, in_permit: LogInPermit, out_permit: LogOutPermit) -> LoggerId {
}

#[macro_export]
macro_rules! log {
    ($mask: expr, $group: expr => $($args: tt)*) => {
        
    };
    ($mask: expr => $($args: tt)*) => {

    };
}


