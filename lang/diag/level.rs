/// The level of the diagnostic
#[derive(Debug, Clone)]
pub enum Level {
    Trace,
    Error,
    Warning,
    Info,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Level::Trace => write!(f, "trace"),
            Level::Error => write!(f, "error"),
            Level::Warning => write!(f, "warning"),
            Level::Info => write!(f, "info"),
        }
    }
}
