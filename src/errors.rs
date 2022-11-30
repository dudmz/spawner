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


#[derive(Debug, Clone)]
pub struct StandaloneServeUnreachableError;

impl std::fmt::Display for StandaloneServeUnreachableError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "unreachable")
    }
}

impl std::error::Error for StandaloneServeUnreachableError {}

#[derive(Debug, Clone)]
pub struct DistributedServePublishRequiredError;

impl std::fmt::Display for DistributedServePublishRequiredError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "publish argument required when running web instance")
    }
}

impl std::error::Error for DistributedServePublishRequiredError {}