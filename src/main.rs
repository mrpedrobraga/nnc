use crate::parser::build_tree;
use colored::Colorize;
use file_importer::import_as_text;
use parser::tokenize;
use std::env;

mod file_importer;
pub mod grammar;
mod parser;

const VERSION: &'static str = "0.0.1";

/// The entry point of the CLI app.
fn main() {
    let args: Vec<String> = env::args().collect();

    // Prints the nano inital screen!
    if args.len() < 2 {
        print_info();
        return;
    }

    print!("\n");
    match args[1].as_str() {
        "version" => print_info(),
        "help" => {
            if args.len() == 2 {
                print_help()
            } else {
                println!("Additional help not yet implemented.")
            }
        }
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
        "run" => println!("Not yet implemented."),
        "repl" => println!("Not yet implemented."),
        "interact" => println!("Not yet implemented."),
        "clean" => println!("Not yet implemented."),
        "test" => println!("Not yet implemented."),
        "lint" => println!("Not yet implemented."),
        "fmt" => println!("Not yet implemented."),
        "lsp" => println!("Not yet implemented."),
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

fn print_info() {
    println!(
        "\n{} - the {} compiler\n{} {}.\n\n{}\n",
        "`nnc`".green().bold(),
        "nano".cyan(),
        "Version:".dimmed(),
        VERSION.cyan(),
        "Use `nnc` help to see a quick manual.".dimmed()
    );
}

/// nnc compile <entry_point_path>
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
    let tree = build_tree(&source, &tokens, "Program", false);

    let tree = match tree {
        None => return None,
        Some(t) => t,
    };

    println!("{:#?}", tree);

    return Some(true);
}

/// nnc help
fn print_help() {
    println!(
        "Usage: `{} {}`.",
        "nnc".green().bold(),
        "<subcommand>".blue().bold()
    );
    println!("");
    println!(
        "{} - Prints the compiler name and version.\n{} {}{}\n",
        "version".bold(),
        "It's currently".dimmed(),
        VERSION.dimmed(),
        "!".dimmed(),
    );
    println!(
        "{} - Shows how to use every subcommand available in nnc, or, optionally, detailed information about a subcommand.\n",
        "help [subcommand]".bold(),
    );
    println!(
        "{} - Begins compilation starting at <entry_file>.\n",
        "compile <entry_file>".bold()
    );
    println!(
        "{} - Same as `compile`, but immediately runs the exported executable.\n",
        "run <entry_file>".bold()
    );
    println!(
        "{} - Creates a new environment to play with nano without creating files.\n",
        "repl".bold()
    );
    println!(
        "{} - Mixture of `run` and `repl`, runs your code and allows interacting with the running project via the REPL and hot-reloading.\n{}\n",
        "interact <entry_file>".bold(),
        "In design stage: It's complicated to say how this can work with compile-time code, which is possibly dangerous.".dimmed()
    );
    println!("{} - Cleans unnecessary cache.\n", "clean".bold());
    println!(
        "{} - Begins testing starting at <entry_file>.\n",
        "test <entry_file>".bold()
    );
    println!("{} - Locates linting configuration in the workspace, then provides ERRORs and WARNINGs for a file.\n", "lint <file>".bold());
    println!(
        "{} - Locates a style configuration and formats the file accordingly.\n",
        "fmt <file>".bold()
    );
    println!(
        "{} - Starts the language server in a given port.\n",
        "lsp <port>".bold()
    );
    println!("{}", "The compilation parameters and flags can be read from your nano source to further alter compilation/testing params.".dimmed())
}

/// Reads a file as text, errors if an error is encountered,
/// and returns its string if all was successful.
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
