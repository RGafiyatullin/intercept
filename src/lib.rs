mod config;
mod filter;

#[ctor::ctor]
#[no_mangle]
fn ctor() {
    let config = if let Some(config) = config::load() {
        // eprintln!("args: {:?}", std::env::args().collect::<Vec<_>>());
        // eprintln!("Config found: {:#?}", config);
        config
    } else {
        return;
    };

    let _ = filter::dispatch(&config);
}

#[ctor::ctor]
#[no_mangle]
fn dtor() {}
