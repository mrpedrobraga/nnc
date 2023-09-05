use std::env;

use colored::Colorize;

use file_importer::import_as_text;
use parser::tokenize;

use crate::parser::build_tree;

mod file_importer;
pub mod grammar;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = "0.0.1";

    if args.len() < 2 {
        println!(
            "\n{} - the {} compiler\n{} {}.\n\n{}\n",
            "`nnc`".green().bold(),
            "nano".cyan(),
            "Version:".dimmed(),
            version.cyan(),
            "Use `nnc` help to see a quick manual.".dimmed()
        );
        return;
    }

    print!("\n");
    match args[1].as_str() {
        "help" => print_help(),
        "compile" => {
            if args.len() < 3 {
                println!(
                    "{}: `nnc compile <entry_file>`\n{}\n",
                    "Usage".bold(),
                    "for example: `nnc compile ./index.nano`".dimmed()
                );
                return;
            }

            let _ = compile(&args);
        }
        _ => {
            println!(
                "Subcommand '{}' not recognized.\nUse `nnc help` to see a quick manual.",
                args[1]
            )
        }
    }
    print!("\n");

    //file_importer::import_nano_source("./examples/hello_world.nano");
}

/* nnc compile <entry_point_path> */
fn compile(args: &Vec<String>) -> Option<bool> {
    // Read text from source file
    let source_path = (args[2]).as_str();
    let source = match read_file(source_path, args) {
        Some(value) => value,
        None => return None,
    };

    // Tokenization: &str -> Vec<Token>
    let tokens = tokenize(source.as_str());

    // Parsing: Vec<Token> -> AST
    let tree = build_tree(&tokens, true);

    let tree = match tree {
        None => return None,
        Some(t) => t,
    };

    println!("{:#?}", tree);

    return Some(true);
}

fn read_file(source_path: &str, args: &Vec<String>) -> Option<String> {
    let source = import_as_text(source_path);
    let source = match source {
        Err(e) => {
            println!(
                "`{} {} -- {}`:\n File '{}' not found:\n → {}\n",
                "nnc".green(),
                "compile".cyan(),
                "FAILED".red(),
                args[2],
                e
            );
            return None;
        }
        Ok(t) => t,
    };
    Some(source)
}

fn print_help() {
    println!(
        "Usage: `{} {}`.",
        "nnc".green().bold(),
        "<subcommand>".blue().bold()
    );
    println!("");
    println!(
        "{}\n - Begins compilation starting at <entry_file>.\n",
        "compile <entry_file>".bold()
    );
    println!(
        "{}\n - Same as `compile`, but immediately runs the exported executable.\n",
        "run <entry_file>".bold()
    );
    println!("{}\n - Cleans unnecessary cache.\n", "clean".bold());
    println!(
        "{}\n - Begins testing starting at <entry_file>.\n",
        "test <entry_file>".bold()
    );
    println!("{}\n - Locates linting configuration in the workspace, then provides ERRORs and WARNINGs for a file.\n", "lint <file>".bold());
    println!(
        "{}\n - Locates a style configuration and formats the file accordingly.\n",
        "fmt <file>".bold()
    );
    println!(
        "{}\n - Starts the language server in a given port.\n",
        "lsp <port>".bold()
    );
    println!("{}", "The compilation parameters and flags can be read from your nano source to further alter compilation/testing params.".dimmed())
}
