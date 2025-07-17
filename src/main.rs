#[cfg(not(feature = "cli"))]
compile_error!("This binary crate requires `--features cli`.");

use clap::{Args, Parser};
use clippy_say::clippy_say;

fn main() {
    let args = CmdLine::parse();
    let text = if args.no_escapes {
        args.text
    } else {
        args.text.replace("\\n", "\n")
    };
    let clippy_says = clippy_say(text);

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

    #[arg(
        short = 'n',
        long,
        help = "Don't parse escape sequences that get passed in",
        default_value_t = false
    )]
    no_escapes: bool,

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
