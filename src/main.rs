use clap::Parser;
use std::fs;

pub mod ast;
pub mod formatter;
pub mod parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let ast = parser::parse_config_to_ast(&cli.input);
    let formatted_output = formatter::format_ast(&ast);

    let out = match cli.output {
        Some(var) => var,
        None => cli.input,
    };

    fs::write(out, formatted_output).expect("Unable to write formatted config");
    let _ = fs::write("log.rs", format!("{:?}", &ast));
}
