use std::env;
use colored::Colorize;

use file_importer::import_as_text;
use parser::tokenize;

use crate::parser::build_tree;

mod parser;
mod file_importer;
pub mod grammar;

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = "0.0.1";

    if args.len() < 2 {
        println!("\n{} - the {} compiler\n{} {}.\n\n{}", "`nnc`".green().bold(), "nano".cyan(), "Version:".dimmed(), version.cyan(), "Use `nnc` help to see a quick manual.".dimmed());
        return
    }
    
    print!("\n");
    match args[1].as_str() {
        "help" => print_help(),
        "compile" => {
            if args.len() < 3 {
                println!("{}: `nnc compile <entry_file>`\n{}", "Usage".bold(), "for example: `nnc compile ./index.nano`".dimmed());
                return
            }

            let source = import_as_text(args[2].as_str());
            
            let source = match source {
                Err(e) => {
                    println!("`{} {} -- {}`:\n File '{}' not found:\n → {}", "nnc".green(), "compile".cyan(), "FAILED".red().bold(), args[2], e);
                    return;
                }
                Ok(t) => t,
            };

            let tokens = tokenize(source.as_str());

            let _ = build_tree(tokens.as_ref());
        }
        _ => {
            println!("Subcommand '{}' not recognized.\nUse `nnc help` to see a quick manual.", args[1])
        }
    }
    print!("\n");

    //file_importer::import_nano_source("./examples/hello_world.nano");
}

fn print_help() {
    println!("Usage: `{} {}`.", "nnc".green().bold(), "<subcommand>".blue().bold());
    println!("");
    println!("{}\n - Begins compilation starting at <entry_file>.\n", "compile <entry_file>".bold());
    println!("{}\n - Same as `compile`, but immediately runs the exported executable.\n", "run <entry_file>".bold());
    println!("{}\n - Cleans unnecessary cache.\n", "clean".bold());
    println!("{}\n - Begins testing starting at <entry_file>.\n", "test <entry_file>".bold());
    println!("{}\n - Locates linting configuration in the workspace, then provides ERRORs and WARNINGs for a file.\n", "lint <file>".bold());
    println!("{}\n - Locates a style configuration and formats the file accordingly.\n", "fmt <file>".bold());
    println!("{}\n - Starts the language server in a given port.\n", "lsp <port>".bold());
    println!("{}", "The compilation parameters and flags can be read from your nano source to further alter compilation/testing params.".dimmed())
}