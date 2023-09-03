use std::env;

use file_importer::import_nano_source;

mod file_importer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = "0.0.1";

    if args.len() < 2 {
        println!("`nnc` - the `nano` compiler\nVersion: '{}'.\n\nUse `nnc` help to see a quick manual.", version);
        return
    }
    
    match args[1].as_str() {
        "help" => print_help(),
        "compile" => {
            if args.len() < 3 {
                println!("Usage: `nnc compile <entry_file>`\nfor example: `nnc compile ./index.nano`")
            }

            let source = import_nano_source(args[2].as_str());
            
            let source = match source {
                Err(e) => {
                    println!("File '{}' not found:\n → {}", args[2], e);
                    return;
                }
                Ok(t) => t,
            };

            println!("{}", source);
        }
        _ => {
            println!("Subcommand '{}' not recognized.\nUse `nnc help` to see a quick manual.", args[1])
        }
    }

    //file_importer::import_nano_source("./examples/hello_world.nano");
}

fn print_help() {
    println!("Usage: `nnc <subcommand>`.");
    println!("");
    println!("compile <entry_file>\n - Begins compilation starting at <entry_file>.\n");
    println!("run <entry_file>\n - Same as `compile`, but immediately runs the exported executable.\n");
    println!("clean\n - Cleans unnecessary cache.\n");
    println!("test <entry_file>\n - Begins testing starting at <entry_file>.\n");
    println!("lint <file>\n - Locates linting configuration in the workspace, then provides ERRORs and WARNINGs for a file.\n");
    println!("fmt <file>\n - Locates a style configuration and formats the file accordingly.\n");
    println!("lsp <port>\n - Starts the language server in a given port.\n");
    println!("The compilation parameters and flags can be read from your nano source to further alter compilation/testing params.\n")
}