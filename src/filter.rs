use std::collections::HashMap;
use std::process::Command;

use crate::config::Config;

pub fn resolve(config: &Config) -> Option<Command> {
    let exe = executable(std::env::args().next()?);
    let argv = std::env::args().collect::<Vec<_>>();
    let env = std::env::vars().collect::<HashMap<_, _>>();

    let interceptor = config.intercepted.get(&exe)?;

    let mut command = std::process::Command::new(interceptor.exe.as_str());
    command
        .args(interceptor.args.iter().chain(argv.iter()))
        .env_clear()
        .envs(env.into_iter().filter(|(k, _)| k != "LD_PRELOAD"));

    Some(command)
}

fn executable(fallback: String) -> String {
    let proc_exe_location = format!("/proc/{}/exe", std::process::id());
    std::fs::read_link(proc_exe_location)
        .map(|p| p.to_string_lossy().as_ref().to_owned())
        .unwrap_or(fallback)
}
