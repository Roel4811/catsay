extern crate structopt;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Catsay", about = "cli app with a talking cat")]
struct Options {
    #[structopt(name = "message", help = "pass a message", default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appear dead
    dead: bool,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;
    if message.to_lowercase() == "woof" {
        eprintln!("Cat doesn't want to bark like a dog")
    }

    let eye = if options.dead { "x" } else { "o" };

    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("    /\\_/\\");
    println!("   ( {eye} {eye} )", eye = eye);
    println!("   =( I )=");
}
