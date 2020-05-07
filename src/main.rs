use std::env;

mod util;

mod generators;
use generators::*;

fn main() {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    let count = args
        .get(0)
        .expect("give arguments")
        .parse::<u128>()
        .expect("give unsigned length");

    args.remove(0);

    let mut gens = args.iter().map(|x| x.as_ref()).collect::<GeneratorVector>();

    for _ in 0..count {
        println!("{}", gens.next_line())
    }
}
