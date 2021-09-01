
mod config;

#[ctor::ctor]
#[no_mangle]
fn ctor() {
    if let Some(config) = config::load() {
        eprintln!("Config found: {:#?}", config);
    } else {
        eprintln!("No config found. Resuming normally.")
    }
}


#[ctor::ctor]
#[no_mangle]
fn dtor() {}

