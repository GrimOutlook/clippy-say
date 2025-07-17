#[cfg(not(feature = "cli"))]
compile_error!("This binary crate requires `--features cli`.");

use clap::{Args, Parser};
use clippy_says::clippy_says;

fn main() {
    let args = CmdLine::parse();
    let clippy_says = clippy_says(args.text);

    if args.output.stderr {
        eprintln!("{}", clippy_says)
    } else {
        println!("{}", clippy_says)
    }
}

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct CmdLine {
    #[arg(help = "What clippy will say")]
    text: String,

    #[command(flatten)]
    output: Output,
}

#[derive(Args)]
#[group(multiple = false)]
struct Output {
    #[arg(
        short = 'o',
        long,
        group = "output",
        help = "Print to stdout",
        default_value_t = true
    )]
    stdout: bool,

    #[arg(short = 'e', long, group = "outout", help = "Print to stderr")]
    stderr: bool,
}
