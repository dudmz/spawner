#[derive(Debug, Clone)]
pub struct StandaloneServeNoopError;

impl std::fmt::Display for StandaloneServeNoopError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "serve command is noop for standalone mode")
    }
}

impl std::error::Error for StandaloneServeNoopError {}

#[derive(Debug, Clone)]
pub struct InvalidCommandError;

impl std::fmt::Display for InvalidCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid crawler command")
    }
}

impl std::error::Error for InvalidCommandError {}

