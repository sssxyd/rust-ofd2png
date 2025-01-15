use std::fs;
use std::io::BufReader;

use rofd::ofd;
use rofd::render;

fn main() {
    std::process::exit(real_main1());
}

fn real_main1() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }

    let _ofd = ofd::Ofd::from_filename(&args[1]).unwrap();
    0
}

fn real_main2() -> i32 {
    render::render();
    0
}