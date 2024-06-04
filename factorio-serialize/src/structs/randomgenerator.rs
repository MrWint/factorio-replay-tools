use factorio_serialize_derive::MapReadWriteStruct;

#[derive(Clone, Debug, MapReadWriteStruct)]
pub struct RandomGenerator {
  pub seed1: u32,
  pub seed2: u32,
  pub seed3: u32,
}
impl RandomGenerator {
  pub const fn new(seed1: u32, seed2: u32, seed3: u32) -> Self {
    RandomGenerator { seed1, seed2, seed3 }
  }

  fn next(&mut self) -> u32 {
    self.seed1 = ((self.seed1 ^ self.seed1 << 13) >> 19) | (self.seed1 >> 1 << 13);
    self.seed2 = ((self.seed2 ^ self.seed2 << 2) >> 25) | (self.seed2 >> 3 << 7);
    self.seed3 = ((self.seed3 ^ self.seed3 << 3) >> 11) | (self.seed3 >> 4 << 21);
    self.seed1 ^ self.seed2 ^ self.seed3
  }

  // from RandomGenerator::uniformInteger
  pub fn uniform_integer(&mut self, min: u32, max: u32) -> u32 {
    return min + self.next() % (max - min);
  }

  // from RandomGenerator::uniformDouble
  pub fn uniform_double(&mut self) -> f64 {
    return self.next() as f64 * (1.0 / 4294967296.0);
  }

  // from ItemProductPrototype::collect
  pub fn get_huge_rock_items(&mut self) -> (u32, u32) { // (stone, coal)
    let stone = (self.uniform_double() * (51.0-24.0) + 24.0) as u32;  // needs to be at least 4_135_894_434 (0xf684bda2) for 50 output
    let coal = (self.uniform_double() * (51.0-24.0) + 24.0) as u32;
    (stone, coal)
  }
}
