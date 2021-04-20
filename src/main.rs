extern crate structopt;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Catsay", about = "cli app with a talking cat")]
struct Options {
    #[structopt(name = "message", help = "pass a message", default_value = "Meow!")]
    /// What does the cat say?
    message: String,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("    /\\_/\\");
    println!("   ( o o )");
    println!("   =( I )=");
}
