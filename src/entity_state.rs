#[derive(Clone, Copy)]
pub enum EntityState<K: Copy> {
    Stationary(K, u32),
    Traversing(K, K, u32),
}
