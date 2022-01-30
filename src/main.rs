use std::env;

fn usage() {
    eprintln!("Usage: passg [ OPTIONS ] [ length ] \n\n\
	       -C Use capitalized letters in the password \n\
	       -c Don't use capitalized letters in the password \n\
	       -N Use numbers in the passowrd \n\
	       -n Don't use numbers in the password \n\
	       -S Use special symbols in the password \n\
	       -s Don't use special symbols in the password \n\n\
	       The default is -CNS with a length of 32.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
	_ => {
	    usage();
	}
    }
}
