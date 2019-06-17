#![deny(warnings, dead_code)]

mod error;
mod util;
mod gen;

use std::{
    io,
    time::Instant,
};

use iota_lib_rs::{
    utils::generate_new_seed,
};

use structopt::StructOpt;
use scoped_pool::Pool;

use crate::error::Error;
use crate::gen::*;


#[derive(StructOpt)]
struct Args {
    #[structopt(short="g", long, help="Automatically generate a seed.")]
    autogen_seed: bool,

    #[structopt(short="t", long, default_value = "0", help="Manually specify number of threads.")]
    num_threads: usize,

    #[structopt(short="n", long, default_value = "1000", help="Specify number of addresses.")]
    num_addresses: usize,

    #[structopt(short="i", long, default_value = "0", help="Specify start index.")]
    start_index: usize,

    #[structopt(short, long, help="Return seed (slower).")]
    return_seed: bool,
}

fn read_seed() -> Result<String, Error> {
    println!("Please enter a seed or just press enter to get a random seed");

    let mut input = String::new();
    let seed: String = loop {
        io::stdin().read_line(&mut input)?;

        let seed = input.trim_end_matches(|c| char::is_control(c) || char::is_whitespace(c));        
        //let seed = input.trim_end().parse::<String>().expect("error");

        if is_valid_seed(&seed) {
            break seed.to_string();
        } else {
            println!("Entered seed was invalid! ({} trytes)", seed.len());
        }

        input.clear();
    };

    Ok(seed)
}

fn is_valid_seed(trytes: &str) -> bool {
    const SEED_LENGTH: usize = 81;
    if trytes.len() != SEED_LENGTH {
        return false;
    }
    is_tryte_string(trytes)
}

fn is_tryte_string(trytes: &str) -> bool {
    trytes.chars().all(|c| c == '9' || (c >= 'A' && c <= 'Z'))
}

fn get_num_threads(args: &Args) -> usize {
    let max_available = num_cpus::get();

    if args.num_threads == 0 {
        max_available
    } else {
        if args.num_threads < max_available {
            args.num_threads
        } else {
            max_available
        }
    }
}

fn get_words() -> Result<Vec<String>, Error> {
    println!("Please enter the words you want to use to search for an address, separated by a space");

    let mut input = String::new();
    let words = loop {
        io::stdin().read_line(&mut input)?;

        let words = input.trim_end_matches(|c| char::is_control(c) || char::is_whitespace(c));        
        let words = words.split_ascii_whitespace().map(|w| w.to_uppercase()).collect::<Vec<_>>();

        if words.iter().all(|w| is_tryte_string(&w)) {
            break words;
        } else {
            println!("Entered words couldn't be converted to trytes");
        }

        input.clear();
    };

    Ok(words)
}

macro_rules! create_process {
    ($func:ident, $call:ident) => {

        fn $func(seed: &str, words: &Vec<String>, num_threads: usize, args: &Args) {
            let pool = Pool::new(num_threads);
            let amount = args.num_addresses / num_threads;

            println!("Start {} threads with {} addresses to generate for each:", num_threads, amount);

            let start = Instant::now();
            pool.scoped(|scope| {
                for i in 0..num_threads {
                    let i = i.clone();
                    scope.execute(move || {
                        let offset = i * amount;
                        $call(offset + args.start_index, offset + amount + args.start_index, seed, words, num_threads, i);
                    })
                }
            });
            let stop = start.elapsed();
            println!("Duration: {:?} for {} addresses", stop, amount * num_threads);
        }
    }
}

create_process!(process_generate_addresses_seed, generate_addresses_seed);
create_process!(process_generate_addresses_prvkey, generate_addresses_prvkey);

fn main() -> Result<(), Error> {
    let args = Args::from_args();

    let seed = if args.autogen_seed {
        generate_new_seed()
    } else {
        read_seed()?
    };
    println!("seed={}", seed);

    if args.num_addresses == 0 {
        return Err(Error::App("num_addresses must be >0."));
    }
    println!("num_addresses = {}", args.num_addresses);
    println!("start_index = {}", args.start_index);

    let words = get_words()?;
    println!("words = {:?}", words);

    let num_threads = get_num_threads(&args);
    println!("num_threads = {}", num_threads);

    //process(&seed, &words, num_threads, &args);
    if args.return_seed {
        process_generate_addresses_seed(&seed, &words, num_threads, &args);
    } else {
        process_generate_addresses_prvkey(&seed, &words, num_threads, &args);
    }

    Ok(())
}