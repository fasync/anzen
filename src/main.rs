use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    capitalized: bool,

    #[clap(short, long)]
    numbers: bool,

    #[clap(short, long)]
    special: bool,

    #[clap(short, long)]
    length: u32,
}

fn main() {
    let args = Args::parse();
    let mut rng = rand::thread_rng();

    const DEFAULT_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const CAPITALIZED_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBER_CHARSET: &[u8] = b"0123456789";
    const SPECIAL_CHARSET: &[u8] = b")(*&^%$#@!~\"\'";

    let mut charset: Vec<u8> = DEFAULT_CHARSET.to_vec();

    if args.capitalized {
        charset.append(&mut CAPITALIZED_CHARSET.to_vec());
    }

    if args.numbers {
        charset.append(&mut NUMBER_CHARSET.to_vec());
    }

    if args.special {
        charset.append(&mut SPECIAL_CHARSET.to_vec());
    }

    let password: String = (0..args.length).map(|_| {
	let index = rng.gen_range(0..charset.len());
	charset[index] as char
    }).collect();

    println!("{}", password);
}
