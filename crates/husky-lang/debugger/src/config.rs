use crate::*;

pub(crate) struct DebuggerConfig {
    pub(crate) verbose: bool,
    pub(crate) opt_input_id: Option<String>,
}

impl DebuggerConfig {
    pub(crate) fn from_env() -> Self {
        let flags = flags::HuskyLangDebuggerCommand::from_env().expect("Invalid environment");
        Self {
            verbose: flags.verbose,
            opt_input_id: flags.input_id,
        }
    }
}
