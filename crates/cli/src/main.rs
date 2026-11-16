mod commands;
mod config;
mod snapshot;

clawless::main!();

/// Returns the handle that the commands write their output to
fn stdout() -> std::io::Stdout {
    std::io::stdout()
}

/// Returns the handle that the commands write their diagnostics to
fn stderr() -> std::io::Stderr {
    std::io::stderr()
}
