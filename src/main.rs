use clap::{App, crate_version};
use std::path::PathBuf;
use std::vec::Vec;

mod filesystem;

#[derive(Debug)]
enum Cmd {
    Add,
    Remove,
    Hook,
    Unhook,
    Status,
    List,
    None
}

#[derive(Debug)]
struct Args {
    command:    Cmd,
    verbose:    bool,
    homedir:    bool,
    recursive:  bool,
    keep:       bool,
    bundle:     PathBuf,
    sub:        Vec<PathBuf>,
    paths:      Vec<PathBuf>,
}

fn main() {

    if cfg!(windows) {
        eprintln!("Windows is unsupported!");
        quit::with_code(127);
    }

    let mut arg_command:    Cmd     = Cmd::None;
    let mut arg_verbose:    bool    = false;
    let mut arg_homedir:    bool    = false;
    let mut arg_recursive:  bool    = false;
    let mut arg_keep:       bool    = false;
    let mut arg_bundle:     PathBuf = filesystem::get_path("~/.local/share/sydf", true);
    let mut arg_sub:        Vec<PathBuf> = Vec::new();
    let mut arg_paths:      Vec<PathBuf> = Vec::new();
    
    let yaml_bytes = std::include_bytes!("clap.yaml");
    let yaml_str = String::from_utf8_lossy(yaml_bytes).to_string();
    let yaml = yaml_rust::YamlLoader::load_from_str(yaml_str.as_str());
    let yaml = match yaml {
        Ok(y) => y[0].clone(),
        Err(error) => {
            eprintln!("{}", error);
            quit::with_code(127);
            
        }
    };

    let matches = App::from(&yaml).version(crate_version!()).get_matches();

    if matches.is_present("debug") {
        arg_verbose = true;
    }

    if matches.is_present("keep-homedir") {
        arg_homedir = true;
    }

    if let Some(b) = matches.value_of("bundle") {
        arg_bundle = filesystem::get_path(&b, true);
    }

    if let Some(s) = matches.value_of("sub-bundle") {
        arg_bundle.push(filesystem::get_path(&s, false));
    }

    if let Some(ref matches) = matches.subcommand_matches("add") {
        arg_command = Cmd::Add;
        if let Some(paths) = matches.values_of("path") {
            for p in paths {
                arg_paths.push(filesystem::get_path(&p, true));
            }
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("rm") {
        arg_command = Cmd::Remove;
        if let Some(paths) = matches.values_of("path") {
            for p in paths {
                arg_paths.push(filesystem::get_path(&p, true));
            }
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("hook") {
        arg_command = Cmd::Hook;
        if let Some(subs) = matches.values_of("sub-bundle") {
            for s in subs {
                arg_sub.push(filesystem::get_path(&s, false));
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
                arg_sub.push(filesystem::get_path(&s, false));
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
        homedir:    arg_homedir,
        recursive:  arg_recursive,
        keep:       arg_keep,
        bundle:     arg_bundle,
        sub:        arg_sub,
        paths:      arg_paths
    };

    println!("{:?}", args);

    // init bundle

    match args.command {
        Cmd::Add => {
            for p in args.paths {
                println!("Add: {}", p.display());
            }
        },
        Cmd::Remove => {
            for p in args.paths {
                println!("Remove: {}", p.display());
            }
        },
        Cmd::Hook => {
            if args.sub.len() == 0 {
                println!("Hook bundle: {}", args.bundle.display());
            } else {
                for s in args.sub {
                    println!("Hook sub-bundle: {}", s.display());
                }
            }
        },
        Cmd::Unhook => {
            if args.sub.len() == 0 {
                println!("Unhook bundle: {}", args.bundle.display());
            } else {
                for s in args.sub {
                    println!("Unhook sub-bundle: {}", s.display());
                }
            }
        },
        Cmd::Status => {
            println!("Status");
        },
        Cmd::List => {
            println!("List");
        },
        Cmd::None => {}
    }

}