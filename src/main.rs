#[derive(Clone, Copy, PartialEq, Eq, borsh::BorshSerialize)]
struct NaiveHasher;
impl core::hash::BuildHasher for NaiveHasher {
  type Hasher = NaiveHasher;
  fn build_hasher(&self) -> NaiveHasher { NaiveHasher }
  fn hash_one<T: core::hash::Hash>(&self, x: T) -> u64 where Self: Sized, Self::Hasher: core::hash::Hasher { 0 }
}
impl core::hash::Hasher for NaiveHasher {
  fn finish(&self) -> u64 { 0 }
  fn write(&mut self, bytes: &[u8]) {}
}
type HashMap = hashbrown::HashMap<u8, u8, NaiveHasher>;
fn main() {
  let mut map_a = HashMap::with_hasher(NaiveHasher);
  map_a.insert(0, 1);
  map_a.insert(2, 3);
  let mut map_b = HashMap::with_hasher(NaiveHasher);
  map_b.insert(2, 3);
  map_b.insert(0, 1);
  assert_eq!(borsh::to_vec(&map_a).unwrap(), borsh::to_vec(&map_b).unwrap());
}
