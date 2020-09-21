pub fn init(p: &std::path::Path) {
    match std::fs::create_dir_all(p.join(".sydf")) {
        Err(e) => {
            eprintln!("Invalid working directory: {}", e);
            quit::with_code(2)
        }
        _ => {}
    };
}


pub fn path() -> std::path::PathBuf {
    match std::env::var_os("SYDF") {
        Some(val) => {
            match val.to_str() {
                Some(p) => return std::path::PathBuf::from(p),
                None => {
                    eprintln!("Unable to parse 'SYDF' environment variable!");
                    quit::with_code(1);
                }
            }
        },
        None => {
            match std::env::var_os("HOME") {
                Some(val) => match val.to_str() {
                    Some(h) => return std::path::PathBuf::from(
                        &format!("{}/.local/share/sydf", h)
                    ),
                    None => {
                        eprintln!("Unable to parse 'HOME' environment variable!");
                        quit::with_code(1);
                    }
                }
                None => {
                    eprintln!("Missing 'HOME' environment variable!");
                    quit::with_code(1);
                }
            }
        }
    }
}