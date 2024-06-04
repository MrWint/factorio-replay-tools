use factorio_serialize::RandomGenerator;

#[allow(dead_code)]
pub fn brute_force_rock_rng() {
  // 500: RandomGenerator { seed1: 80686, seed2: 3738370905, seed3: 872480768 }
  // 598: RandomGenerator::new(1510092962, 2768646422, 1113768448)
  // 692: RandomGenerator { seed1: 1876885255, seed2: 2951610103, seed3: 4294836223 }
  // 960: RandomGenerator { seed1: 4225719483, seed2: 1869606895, seed3: 4294967295 }
  let mut current_rng = RandomGenerator::new(u32::MAX, u32::MAX, u32::MAX);
  let mut current_score = get_huge_rock_score(current_rng.clone());
  loop {
    let mut best_score = current_score;
    let mut best_perm = 0;
    for bit_flips in 1..=5 {
      let mut perm = (1u128 << bit_flips) - 1;
      while perm < (1u128 << 88) {
        let test_score = get_huge_rock_score(nudge_rng(&current_rng, perm));
        if test_score > best_score {
          best_perm = perm;
          best_score = test_score;
        }
        perm = next_perm(perm);
      }
    }
    if best_score == current_score { break; }
    current_score = best_score;
    current_rng = nudge_rng(&current_rng, best_perm);
    println!("found score {current_score} with {current_rng:?}")
  }
}

pub fn next_perm(v: u128) -> u128{
  let t = v | (v - 1); // t gets v's least significant 0 bits set to 1
  // Next set to 1 the most significant bit to change, 
  // set to 0 the least significant ones, and add the necessary 1 bits.
  return (t + 1) | (((!t & (t+1)) - 1) >> (v.trailing_zeros() + 1))
}
fn nudge_rng(rng: &RandomGenerator, perm: u128) -> RandomGenerator {
  // lower 1, 3 and 4 bits respectively don't matter
  let mut seed = (rng.seed1 as u128 >> 1) | ((rng.seed2 as u128) >> 3 << 31) | ((rng.seed3 as u128) >> 4 << 60);
  seed ^= perm;
  RandomGenerator { seed1: (seed << 1) as u32, seed2: (seed >> 31 << 3) as u32, seed3: (seed >> 60 << 4) as u32 }
}
fn get_huge_rock_score(mut rng: RandomGenerator) -> u32 {
  let mut item_count = 0;
  for _ in 0..10 {
    let (stone, coal) = rng.get_huge_rock_items();
    item_count += stone + coal;
  }
  item_count
}

#[allow(dead_code)]
pub fn check_rng_cycles() {
  let mut bitmap = vec![0u8; 1 << 29];
  for start_val in 0..=u32::MAX {
    if bitmap[start_val as usize >> 3] & (1 << (start_val & 7)) == 0 {
      let mut seed: u32 = start_val;
      let mut len = 0;
      while bitmap[seed as usize >> 3] & (1 << (seed & 7)) == 0 {
        bitmap[seed as usize >> 3] |= 1 << (seed & 7);
        len += 1;
        // seed = ((seed ^ seed << 13) >> 19) | (seed >> 1 << 13);
        seed = ((seed ^ seed << 2) >> 25) | (seed >> 3 << 7);
        // seed = ((seed ^ seed << 3) >> 11) | (seed >> 4 << 21);
      }
      if len > 1 {
        println!("start {start_val:x} cycle len: {len:x}");
      }
    }
  }
}