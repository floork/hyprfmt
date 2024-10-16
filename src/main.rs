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

#[derive(Clone)]
pub struct Config {
    pub indentation: u8,
    pub align_comments: bool,
}

fn main() {
    let cli = Cli::parse();

    let ast = parser::parse_config_to_ast(&cli.input);

    let config = Config {
        indentation: 10,
        align_comments: true,
    };

    let formatted_output = formatter::format_ast(&ast, config);

    let out = match cli.output {
        Some(var) => var,
        None => String::from("./out.conf"),
    };

    fs::write(out, formatted_output).expect("Unable to write formatted config");

    let debug = format!("{:?}", &ast);
    fs::write("./debug.log", debug).expect("Unable to write formatted config");
}
