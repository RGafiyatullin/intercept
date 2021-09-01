use std::{collections::HashMap, process::Command};

use crate::config::Config;

pub fn resolve(config: &Config) -> Option<Command> {
    let exe = executable();
    let argv = std::env::args().collect::<Vec<_>>();
    let env = std::env::vars().collect::<HashMap<_, _>>();

    let interceptor = config.intercepted.get(&exe)?;

    let mut command = std::process::Command::new(interceptor.exe.as_str());
    command
        .args(interceptor.args.iter().chain(argv.iter()))
        .envs(env.into_iter().filter(|(k, _)| k != "LD_PRELOAD"));

    Some(command)
}

fn executable() -> String {
    unimplemented!()
}
