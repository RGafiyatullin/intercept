

#[ctor::ctor]
#[no_mangle]
fn ctor() {
    println!("=== CTOR ===");
    
    let args = std::env::args().collect::<Vec<_>>();

    println!("arg: {:?}", std::env::args().collect::<Vec<_>>());

    if std::env::args().any(|a| a.starts_with("hang")) {
        println!("Intercepting...");
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("Terminating...");

        std::process::exit(0);
    }
    
    println!("=== ==== ===");
}


#[ctor::ctor]
#[no_mangle]
fn dtor() {
    println!("=== DTOR ===");
}

