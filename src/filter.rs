use std::collections::HashMap;
use std::os::unix::prelude::CommandExt;
use std::process::Command;

use crate::config::Config;
use crate::config::Interceptor;

pub fn dispatch(config: &Config) -> Option<()> {
    let this_pid = std::process::id().to_string();

    if std::env::var(config.cookie.as_str()).ok().as_ref() == Some(&this_pid) {
        return None;
    }

    let exe = executable(std::env::args().next()?);
    let argv = std::env::args().collect::<Vec<_>>();
    let env = std::env::vars().collect::<HashMap<_, _>>();

    match config.intercepted.get(&exe)? {
        Interceptor::Replace(replace) => {
            let mut command = Command::new(replace.exe.as_str());
            command
                .args(replace.args.iter().chain(argv.iter()))
                .env_clear()
                .env(config.cookie.as_str(), this_pid)
                .envs(env.into_iter().filter(|(k, _)| k != "LD_PRELOAD"));
            let error = command.exec();
            eprintln!("error executing command: {:#?}", error);
            return Some(());
        }
        Interceptor::Report(report) => {
            let mut command = Command::new(report.exe.as_str());
            command
                .args(
                    vec![&this_pid]
                        .into_iter()
                        .chain(report.args.iter())
                        .chain(argv.iter()),
                )
                .env_clear()
                .env(config.cookie.as_str(), this_pid)
                .envs(env.into_iter().filter(|(k, _)| k != "LD_PRELOAD"));
            let _ = command.spawn().ok()?.wait().ok()?;
            return Some(());
        }
    }
}

fn executable(fallback: String) -> String {
    let proc_exe_location = format!("/proc/{}/exe", std::process::id());
    std::fs::read_link(proc_exe_location)
        .map(|p| p.to_string_lossy().as_ref().to_owned())
        .unwrap_or(fallback)
}
