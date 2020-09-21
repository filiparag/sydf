mod workdir;


fn path(path: &str) -> std::path::PathBuf {
    npath::normalize(
        std::path::PathBuf::from(
            shellexpand::tilde(path).to_string()
        )
    )
}

fn main() {

    if cfg!(windows) {
        eprintln!("Windows is unsupported!");
        quit::with_code(127);
    }

    let wd = workdir::path();

    workdir::init(&wd);

}