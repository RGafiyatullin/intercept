use std::os::unix::prelude::CommandExt;

mod config;
mod filter;

#[ctor::ctor]
#[no_mangle]
fn ctor() {
    let config = if let Some(config) = config::load() {
        eprintln!("Config found: {:#?}", config);
        config
    } else {
        return;
    };

    if let Some(mut command) = filter::resolve(&config) {
        let error = command.exec();
        eprintln!("error executing interceptor: {:?}", error);
    }
}

#[ctor::ctor]
#[no_mangle]
fn dtor() {}
