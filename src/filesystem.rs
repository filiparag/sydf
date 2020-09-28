use std::path::PathBuf;
use std::fs;

#[derive(Debug)]
pub enum Type {
    File,
    Directory,
    Symlink,
    Other
}

pub fn get_path(path: &str, abs: bool) -> PathBuf {
    let mut p = PathBuf::from(
        shellexpand::tilde(path).to_string()
    );
    if abs && !p.is_absolute() {
        let mut ap: PathBuf;
        match std::env::current_dir() {
            Ok(pwd) => ap = pwd,
            Err(err) => {
                eprintln!("{}", err);
                quit::with_code(1);
            } 
        }
        ap.push(p);
        p = ap;
    }
    return npath::normalize(p);
}

trait GenericPath {
    fn generic_home(&self) -> PathBuf;
}

impl GenericPath for std::path::PathBuf {
    fn generic_home(&self) -> PathBuf {
        let home = get_path("~", true);
        let mut path = self.clone();
        if path.starts_with(&home) {
            let mut gp = PathBuf::from("HOME");
            match path.strip_prefix(&home) {
                Ok(p) => gp.push(p),
                Err(err) => {
                    eprintln!("{}", err);
                    quit::with_code(1);
                }
            }
            path = gp;
        }
        return npath::normalize(path);
    }
}


pub fn get_type(path: &PathBuf) -> Type {
    match fs::symlink_metadata(path) {
        Ok(attr) => {
            println!("{:?}", attr.file_type());
            if attr.is_dir() {
                return Type::Directory
            } else if attr.is_file() {
                return Type::File
            } else if attr.file_type().is_symlink() {
                return Type::Symlink
            } else {
                return Type::Other
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            quit::with_code(254);
        }
    }
}

pub fn create_dir(path: &PathBuf) {
    match std::fs::create_dir_all(path) {
        Err(e) => {
            eprintln!("{}", e);
            quit::with_code(2)
        }
        _ => {}
    };
}

fn get_target(source: &PathBuf, bundle: &PathBuf, homedir: bool) -> (PathBuf, PathBuf) {
    let mut rel_target = match homedir {
        true => source.clone(),
        false => source.generic_home(),
    };
    if rel_target.is_absolute() {
        match rel_target.strip_prefix("/") {
            Ok(p) => rel_target = p.to_path_buf(),
            Err(e) => {
                eprintln!("{}", e);
                quit::with_code(2)
            }
        }
    }
    let mut dir = bundle.clone();
    dir.push(
        match rel_target.parent() {
            Some(p) => p.to_path_buf(),
            None => quit::with_code(2)
        }
    );
    let mut target = bundle.clone();
    target.push(rel_target);
    return (target, dir)
}

pub fn add(source: &PathBuf, bundle: &PathBuf, homedir: bool) {
    let (target, dir) = get_target(source, bundle, homedir);
    println!("mkdir '{}'", dir.display());
    println!("mv '{}' '{}'", source.display(), target.display());
    println!("ln -s '{}' '{}'", target.display(), source.display());
}

pub fn rm(source: &PathBuf, bundle: &PathBuf, homedir: bool, keep: bool) {
    let (target, _) = get_target(source, bundle, homedir);
    println!("rm '{}'", source.display());
    if keep {
        println!("cp -rp '{}' '{}'", target.display(), source.display());
    } else {
        println!("mv '{}' '{}'", target.display(), source.display());
    }
}