use iota_lib_rs::{
    crypto::HashMode,
    crypto::iss,
    utils::converter,
};

use crate::util::*;

pub(crate) fn generate_addresses_seed (start: usize, end: usize, seed: &str, target_words: &Vec<String>, threadnumber: usize, current_thread: usize){
    let seed_trits = converter::trits_from_string(&seed);

    let mut targetlist = Vec::new();

    for p in target_words {
        targetlist.push(converter::trits_from_string(p))
    }

    for index in start..end {

        if index % 1000 == 0 && current_thread == threadnumber - 1 {
            println!("Addresses remaining: ~{}", threadnumber * (end - index));
        }

        let mut subseed = iss::subseed(HashMode::Kerl, &seed_trits, index).unwrap();
        let key = iss::key(HashMode::Kerl, &mut subseed, 2).unwrap();
        let mut digest = iss::digests(HashMode::Kerl, &key).unwrap();
        let address = iss::address(HashMode::Kerl, &mut digest).unwrap();

        for k in 0..targetlist.len() {
            if targetlist[k] == &address[..targetlist[k].len()]{
                println!("Address found at index {}: {}", index,   converter::trits_to_string(&address).unwrap());
                let new_seed = index_zero_seed(&seed_trits, index);
                println!("New seed: {}", new_seed);
                println!("Privatekey: {}", converter::trits_to_string(&key).unwrap());
            }
        }
    }
}

pub(crate) fn generate_addresses_prvkey (start: usize, end: usize, seed: &str, target_words: &Vec<String>, threadnumber: usize, current_thread: usize){
    let trits = [-1, 0, 1];

    let mut seed_trits = converter::trits_from_string(&seed);
    let mut trit_vec = vec![0; 12879];

    trit_vec.append(&mut seed_trits);

    let mut targetlist = Vec::new();

    for p in target_words {
        targetlist.push(converter::trits_from_string(p))
    }

    for index in start..end {

        if index % 1000 == 0 && current_thread == threadnumber-1 {
            println!("Addresses remaining: ~{}", threadnumber * (end - index));
        }

        let key = random_privatekey(&trits, &trit_vec, index);
        let mut digest = iss::digests(HashMode::Kerl, &key).unwrap();
        let address = iss::address(HashMode::Kerl, &mut digest).unwrap();

        for k in 0..targetlist.len() {
            if targetlist[k] == &address[..targetlist[k].len()]{
                println!("Address found at index {}: {}", index, converter::trits_to_string(&address).unwrap());
                println!("Privatekey: {:?}",converter::trits_to_string(&key).unwrap());
            }
        }
    }
}