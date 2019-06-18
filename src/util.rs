use iota_lib_rs::utils::{converter, trit_adder};

pub(crate) fn random_privatekey(_arr: &[i8; 3], trits: &Vec<i8>, index: usize) -> Vec<i8> {
  let mut trits = trits.clone();

  int2trits(index as i64, &mut trits);

  trits.to_vec()
}

pub(crate) fn index_zero_seed(seed_trits: &[i8], index: usize) -> String {
  let mut index_trits = [0_i8; 243];

  int2trits(index as i64, &mut index_trits);

  let new_seed_trits = trit_adder::add(seed_trits, &index_trits);
  let new_seed = converter::trytes(&new_seed_trits);

  new_seed
}

fn int2trits(v: i64, out: &mut [i8]) {
  let size = out.len();
  let negative = v < 0;
  let mut value = if negative { -v } else { v };
  for i in 0..size {
    if value == 0 {
      break;
    }
    let mut trit = ((value + 1) % (3 as i64)) as i8 - 1;
    if negative {
      trit = -trit;
    }
    out[i] = trit;
    value = (value + 1) / (3 as i64);
  }
}