use clap::Parser;

#[derive(Parser, Debug)]
enum Command {
    GenSyntax,
}

fn main() {
    let cmd = Command::parse();
    match cmd {
        Command::GenSyntax => {
            xtask::gen_syntax::gen_syntax();
        },
    }
}
