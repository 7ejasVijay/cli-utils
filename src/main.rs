use clap::Parser;
use std::fs;

fn main() {
    let cli = Cli::parse();

    if let Some(echo) = cli.echo.as_deref() {
        echo_display(echo);
    }
    if let Some(cat) = cli.cat.as_deref() {
        cat_display(cat);
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Print the text to the standard output
    #[arg(long)]
    echo: Option<String>,
    /// Print the text from a file to standard output
    #[arg(long)]
    cat: Option<String>,
}

fn echo_display(text: &str) {
    println!("{text}");
}

fn cat_display(path: &str) {
    let contents = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => String::from("Error opening the file!"),
    };
    println!("{contents}");
}
