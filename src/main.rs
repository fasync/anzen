/*
 * Copyright (c) 2022, Florian Büstgens
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *     1. Redistributions of source code must retain the above copyright
 *        notice, this list of conditions and the following disclaimer.
 *
 *     2. Redistributions in binary form must reproduce the above copyright notice,
 *        this list of conditions and the following disclaimer in the
 *        documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY Florian Büstgens ''AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL Florian Büstgens BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

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
