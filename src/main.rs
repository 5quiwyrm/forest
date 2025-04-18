mod forest_runtime;
use forest_runtime::*;
mod compile;
use compile::compile;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), ()> {
    let mut args = env::args();
    args.next().expect("Something went horribly wrong - there should be at least one argument being the name of the program");
    match args.next() {
        Some(a) => match a.as_str() {
            "run" => {}
            s => {
                eprintln!("Unknown option {s} - try running `forest`");
                return Err(());
            }
        },
        None => {
            println!(
                "\
                Welcome to the forest compiler!\n\n\
                info: Usage: forest [command] {{options}}\n\n\
                Commands:\n\
                  run  | run from source\n\n\
                Options:\n\
                  dump | dump stack during runtime\n\
                "
            );
            return Ok(());
        }
    }
    if let Some(filepath) = args.next() {
        if let Ok(mut file) = File::open(&filepath) {
            let mut program = String::new();
            match file.read_to_string(&mut program) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Could not read file, reason: {e}");
                    return Err(());
                }
            };
            let instrs = compile(&program)
                .map_err(|err| {
                    eprintln!("Error in compilation: {err}");
                    return 1;
                })
                .unwrap();
            let mut runtime = ForestRuntime::new(&instrs);
            let dump = match args.next() {
                Some(o) => {
                    if o == "dump" {
                        true
                    } else {
                        eprintln!("Unknown option {o}");
                        return Err(());
                    }
                }
                _ => false,
            };
            execute_runtime!(runtime, dump);
        } else {
            eprintln!("Cannot find {filepath} in current directory");
            return Err(());
        }
    } else {
        eprintln!("Please provide a file name!");
        return Err(());
    }
    Ok(())
}
