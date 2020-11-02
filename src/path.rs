use std::path::PathBuf;
use path_abs::PathAbs;

const TILDE: &str = "~";
const RELHOME: &str = "/HOME";

pub fn norm_from(path: &str) -> PathBuf {
    let path = shellexpand::tilde(path).into_owned();
    let abspath = PathAbs::new(&path);
    let abspath = match abspath {
        Ok(p) => PathBuf::from(p),
        Err(_) => PathBuf::from(path)
    };
    abspath
}

pub fn agnostic(path: &PathBuf) -> PathBuf {
    let h = shellexpand::tilde(TILDE).into_owned();
    if path.starts_with(&h) {
        let relpath = path.strip_prefix(&h).unwrap();
        let mut abspath = PathBuf::from(RELHOME);
        if relpath.to_str().unwrap() != "" {
            abspath.push(relpath);
        }
        abspath
    } else {
        path.clone()
    }
}

pub fn real(path: &PathBuf) -> PathBuf {
    let h = PathBuf::from(RELHOME);
    if path.starts_with(&h) {
        let relpath = path.strip_prefix(RELHOME).unwrap();
        let h = shellexpand::tilde(TILDE).into_owned();
        let mut abspath = PathBuf::from(h);
        abspath.push(relpath);
        abspath
    } else {
        path.clone()
    }
}

pub fn to_string(path: &PathBuf) -> String {
    path.as_path().display().to_string()
}