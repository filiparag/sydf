use clap::{App, load_yaml, crate_version};
use std::path::PathBuf;
use std::vec::Vec;

fn path(path: &str) -> std::path::PathBuf {
    npath::normalize(
        std::path::PathBuf::from(
            shellexpand::tilde(path).to_string()
        )
    )
}

#[derive(Debug)]
enum Cmd {
    None,
    Add,
    Remove,
    Hook,
    Unhook,
    Status,
    List
}

#[derive(Debug)]
struct Args {
    command:    Cmd,
    verbose:    bool,
    recursive:  bool,
    keep:       bool,
    bundle:     PathBuf,
    sub:        Vec<PathBuf>,
    paths:      Vec<PathBuf>,
}

fn main() {

    let mut arg_command:    Cmd     = Cmd::None;
    let mut arg_verbose:    bool    = false;
    let mut arg_recursive:  bool    = false;
    let mut arg_keep:       bool    = false;
    let mut arg_bundle:     PathBuf = path("~/.local/share/sydf");
    let mut arg_sub:        Vec<PathBuf> = Vec::new();
    let mut arg_paths:      Vec<PathBuf> = Vec::new();
    
    println!("{}", arg_bundle.display());
    
    let yaml = load_yaml!("clap.yaml");
    let matches = App::from(yaml).version(crate_version!()).get_matches();

    if matches.is_present("debug") {
        arg_verbose = true;
    }

    if let Some(b) = matches.value_of("bundle") {
        arg_bundle = path(&b);
    }

    if let Some(s) = matches.value_of("sub-bundle") {
        arg_bundle.push(path(&s));
    }

    if let Some(ref matches) = matches.subcommand_matches("add") {
        arg_command = Cmd::Add;
        if let Some(paths) = matches.values_of("path") {
            for p in paths {
                arg_paths.push(path(&p));
            }
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("rm") {
        arg_command = Cmd::Remove;
        if let Some(paths) = matches.values_of("path") {
            for p in paths {
                arg_paths.push(path(&p));
            }
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("hook") {
        arg_command = Cmd::Hook;
        if let Some(subs) = matches.values_of("sub-bundle") {
            for s in subs {
                arg_sub.push(path(&s));
            }
        }
        if matches.is_present("recursive") {
            arg_recursive = true;
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("unhook") {
        arg_command = Cmd::Unhook;
        if let Some(subs) = matches.values_of("sub-bundle") {
            for s in subs {
                arg_sub.push(path(&s));
            }
        }
        if matches.is_present("recursive") {
            arg_recursive = true;
        }
        if matches.is_present("keep-content") {
            arg_keep = true;
        }
    }
    
    match matches.subcommand_name() {
        Some("status") => arg_command = Cmd::Status,
        Some("list") => arg_command = Cmd::List,
        None => {},
        _ => {},
    }

    let args = Args {
        command:    arg_command,
        verbose:    arg_verbose,
        recursive:  arg_recursive,
        keep:       arg_keep,
        bundle:     arg_bundle,
        sub:        arg_sub,
        paths:      arg_paths
    };

    println!("{:?}", args);

    match args.command {
        _ => {

        }
    }

}