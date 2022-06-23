pub struct Logger {
    log_level: LogLevel
}

pub const LOGGER : Logger = Logger::new();

impl Logger {
    pub const fn new() -> Logger { 
        Logger {
            log_level: LogLevel::Debug,
        }
    }

    #[inline(always)]
    pub fn info(&self, message: &str) {
        if self.log_level.is_value_of(1) {
            println!("Info: {}", message);
        }
    }

    #[inline(always)]
    pub fn debug(&self, message: &str) {
        if self.log_level.is_value_of(0) {
            println!("Debug: {}", message);
        }
    }

    #[inline(always)]
    pub fn set_log_level(&mut self, level: LogLevel) {
        self.log_level = level;
    }
}

#[derive(PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

impl LogLevel {
    fn value(&self) -> u8 {
        match self {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Warning => 2,
            LogLevel::Error => 3,
            LogLevel::Fatal => 4,
        }
    }

    #[inline(always)]
    fn is_value_of(&self, value: u8) -> bool {
        self.value() <= value
    }
}