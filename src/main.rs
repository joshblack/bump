use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

// Two commands:
// default command (changeset)
// changeset
// version
// publish

fn main() {
    let args = Args::parse();

    println!("{:?}", args)
}
