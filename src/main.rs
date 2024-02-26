use clap::Parser;
use std::fs;

fn main() {
    let cli = Cli::parse();

    cli.cat();
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    echo: Option<String>,
    #[arg(long)]
    cat: String,
}

impl Cli {
    // fn echo(&self) {
    //     println!("{}", self.echo.unwrap());
    // }
    fn cat(&self) {
        let contents = fs::read_to_string(&self.cat).expect("The file cannot be opened!");
        println!("{contents}");
    }
}
